use crate::file_stream;
use crate::Lexer;
use file_stream::StringStream;
use util::error::ZXError;
use util::token::{Literal, Position, Token, Tokens};

pub fn is_whitespace(c: char) -> bool {
    // This is Pattern_White_Space.
    //
    // Note that this set is stable (ie, it doesn't change with different
    // Unicode versions), so it's ok to just hard-code the values.

    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

impl Lexer {
    pub fn lex_identifier(&mut self, string_stream: &mut StringStream) -> Result<(), ZXError> {
        let mut ident = String::from(string_stream.get_currently());
        let start = string_stream.index;

        while !string_stream.is_eof {
            match string_stream.first() {
                c if is_whitespace(c) => break,
                ' '..='/' | ':'..='@' | '['..='^' | '{'..='~' | '`' => break,
                c @ _ => {
                    string_stream.next();
                    ident.push(c)
                }
            }
        }

        self.tokens.push(Token {
            token_type: Tokens::IdentifierToken {
                literal: ident.clone()
            },
            pos: Position {
                start: start,
                end: start + ident.len(),
            },
        });
        Ok(())
    }

    // lex string `"abc\"\n"`
    pub fn lex_string(&mut self, string_stream: &mut StringStream) -> Result<(), ZXError> {
        let mut string_content = String::new();
        let mut end_double_quotes = false;
        let start = string_stream.index.clone();
        string_stream.next();

        while !string_stream.is_eof {
            match string_stream.get_currently() {
                // close string
                '"' => {
                    end_double_quotes = true;
                    break;
                }
                // string escapes
                '\\' => {
                    string_stream.next();
                    let currently = string_stream.get_currently();

                    if !string_stream.is_eof {
                        string_content.push(self.escapes(currently));
                    } else {
                        break;
                    }
                }
                // string content
                _ => {
                    string_content.push(string_stream.get_currently());
                }
            };

            string_stream.next();
        }

        if end_double_quotes {
            self.tokens.push(Token {
                token_type: Tokens::LiteralToken {
                    kid: Literal::String,
                    literal: string_content,
                },
                pos: Position {
                    start,
                    end: string_stream.index,
                },
            });

            Ok(())
        } else {
            Err(ZXError::SyntaxError {
                message: "EOL while scanning string literal".to_string(),
                pos: Position { start, end: start },
            })
        }
    }
    // lex char `'a'`
    pub fn lex_char(&mut self, string_stream: &mut StringStream) -> Result<(), ZXError> {
        let start = string_stream.index.clone();
        let mut c = String::new();
        let mut end_apostrophe = false;

        string_stream.next();

        while !string_stream.is_eof {
            match string_stream.get_currently() {
                // close char
                '\'' => {
                    end_apostrophe = true;
                    break;
                }
                // char escapes
                '\\' => {
                    string_stream.next();
                    let next_char = string_stream.get_currently();
                    c.push(self.escapes(next_char));
                }
                // char content
                _ => {
                    c.push(string_stream.get_currently());
                }
            }

            string_stream.next();
        }

        let error_message = if !end_apostrophe {
            "EOL while scanning char literal"
        } else if c.len() > 1 {
            "character literal may only contain one codepoint"
        } else if c.is_empty() {
            "empty character literal"
        } else {
            ""
        };

        if !error_message.is_empty() {
            Err(ZXError::SyntaxError {
                message: error_message.to_string(),
                pos: Position {
                    start,
                    end: start + c.len() + 1,
                },
            })
        } else {
            self.tokens.push(Token {
                token_type: Tokens::LiteralToken {
                    kid: Literal::Char,
                    literal: c.clone(),
                },
                pos: Position {
                    start,
                    end: start + c.len() + 1,
                },
            });
            Ok(())
        }
    }

    pub fn lex_slash(&mut self, string_stream: &mut StringStream) -> Result<(), ZXError> {
        let start = string_stream.index;

        match string_stream.first() {
            // single line comment
            '/' => {
                string_stream.next();
                while !string_stream.is_eof && string_stream.get_currently() != '\n' {
                    string_stream.next();
                }
                Ok(())
            }
            // multi line comment
            '*' => {
                string_stream.next();
                string_stream.next();
                let mut end_comment = false;

                // search close chars `*/`
                while !string_stream.is_eof {
                    if string_stream.get_currently() == '*' {
                        if string_stream.first() == '/' {
                            end_comment = true;
                            string_stream.next();
                            break;
                        }
                    }
                    string_stream.next();
                }

                if end_comment {
                    Ok(())
                } else {
                    Err(ZXError::SyntaxError {
                        message: "EOL while scanning comment".to_string(),
                        pos: Position {
                            start,
                            end: start + 1,
                        },
                    })
                }
            }
            // slash
            _ => {
                self.tokens.push(Token {
                    token_type: Tokens::SlashToken,
                    pos: Position {
                        start: string_stream.index,
                        end: string_stream.index,
                    },
                });

                Ok(())
            }
        }
    }

    pub fn lex_number(&mut self, string_stream: &mut StringStream) -> Result<(), ZXError> {
        let mut is_folat = false;
        let mut number_string = String::new();
        let currently = string_stream.get_currently();
        let start = string_stream.index;

        number_string.push(currently);

        while !string_stream.is_eof {
            match string_stream.first() {
                '.' => {
                    string_stream.next();
                    match string_stream.first() {
                        '0'..='9' => {
                            is_folat = true;
                            number_string.push(string_stream.get_currently());
                        }
                        _ => break
                    }
                }
                '0'..='9' => {
                    string_stream.next();
                    // println!("{}", string_stream.get_currently());
                    number_string.push(string_stream.get_currently());
                }
                _ => break
            }
        }

        let pos = Position {
            start,
            end: start + number_string.len(),
        };

        let token_type = Tokens::LiteralToken {
            kid: if is_folat {
                Literal::Float
            } else {
                Literal::Integer
            },
            literal: number_string.clone(),
        };

        if (is_folat && number_string.len() > 1) || (!is_folat && number_string.len() > 0) {
            self.tokens.push(Token { token_type, pos });
            match string_stream.get_currently() {
                '.' => {
                    self.tokens.push(Token {
                        token_type: Tokens::DotToken,
                        pos: Position {
                            start: string_stream.index,
                            end: string_stream.index,
                        }
                    })
                }
                _ => {}
            }
            Ok(())
        } else {
            Err(ZXError::SyntaxError {
                message: "".to_string(),
                pos: Position {
                    start,
                    end: start + number_string.len(),
                },
            })
        }
    }
}
