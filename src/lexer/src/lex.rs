use util::token::{Literal, Position, Token, Tokens};
use file_stream::StringStream;
use crate::Lexer;
use crate::file_stream;

impl Lexer {
    // lex string `"abc\"\n"`
    pub fn lex_string(&mut self, string_stream: &mut StringStream) -> Result<(), ()> {
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
            self.push_syntax_error("EOL while scanning string literal", Position {
                start,
                end: start,
            });

            Err(())
        }
    }
    // lex char `'a'`
    pub fn lex_char(&mut self, string_stream: &mut StringStream) -> Result<(), ()> {
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
            self.push_syntax_error(error_message, Position {
                start,
                end: start + c.len() + 1,
            });

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn lex_slash(&mut self, string_stream: &mut StringStream) -> Result<(), ()> {
        let start = string_stream.index;
        string_stream.next();

        match string_stream.get_currently() {
            // single line comment
            '/' => {
                while !string_stream.is_eof && string_stream.get_currently() != '\n' {
                    string_stream.next();
                }
                Ok(())
            }
            // multi line comment
            '*' => {
                string_stream.next();
                let mut end_comment = false;

                // search close chars `*/`
                while !string_stream.is_eof {
                    if string_stream.get_currently() == '*' {
                        string_stream.next();

                        if string_stream.get_currently() == '/' {
                            end_comment = true;
                            string_stream.next();
                            break;
                        } else {
                            string_stream.back();
                        }
                    }
                    string_stream.next();
                }

                if end_comment {
                    Ok(())
                } else {
                    self.push_syntax_error("EOL while scanning comment", Position {
                        start,
                        end: start + 1,
                    });

                    Err(())
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

                string_stream.back();
                Ok(())
            }
        }
    }

    pub fn lex_number(&mut self, string_stream: &mut StringStream) {}
}