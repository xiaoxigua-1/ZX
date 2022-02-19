mod file_stream;
mod test;

use std::fs;
use file_stream::StringStream;
use util::repost::{Level, Repost};
use util::error::ZXError;
use util::token::{Literal, Position, Token, Tokens};

struct Lexer {
    path: String,
    tokens: Vec<Token>,
    reposts: Vec<Repost>,
}

impl Lexer {
    fn new(path: String) -> Lexer {
        Lexer {
            path,
            tokens: vec![],
            reposts: vec![],
        }
    }

    fn lexer(&mut self) -> Result<(), ()> {
        let file_string = fs::read_to_string(self.path.clone()).expect("Something went wrong reading the file");
        let mut file_stream = StringStream::new(&file_string);
        let mut identifier_string = String::new();

        while !file_stream.is_eof {
            let currently = file_stream.get_currently();

            match currently {
                ' '..='/' | ':'..='@' | '['..='`' | '{'..='~' | '\n' | '\r' => {
                    if !identifier_string.is_empty() {
                        self.tokens.push(Token {
                            token_type: Tokens::IdentifierToken {
                                literal: identifier_string
                            },
                            pos: Position {
                                start: file_stream.index,
                                end: file_stream.index,
                            },
                        });
                        identifier_string = String::new();
                    }
                }
                _ => {}
            }

            match currently {
                '"' | '/' => {
                    let result = match currently {
                        '"' => self.lex_string(&mut file_stream),
                        '/' => self.lex_slash(&mut file_stream),
                        _ => Result::Ok(())
                    };

                    match result {
                        Ok(()) => {}
                        Err(()) => {
                            break;
                        }
                    }
                }
                '!'..='.' | ':'..='@' | '['..='`' | '{'..='~' | '\n' => {
                    let kid = match currently {
                        '\n' => Tokens::LineSeparator,
                        '*' => Tokens::MultiplyToken,
                        '+' => Tokens::PlusToken,
                        '-' => Tokens::MinusToken,
                        '>' => Tokens::MoreToken,
                        '<' => Tokens::LessToken,
                        '=' => Tokens::EqualToken,
                        '{' => Tokens::LeftCurlyBracketsToken,
                        '}' => Tokens::RightCurlyBracketsToken,
                        '[' => Tokens::LeftSquareBracketsToken,
                        ']' => Tokens::RightSquareBracketsToken,
                        '(' => Tokens::LeftParenthesesToken,
                        ')' => Tokens::RightParenthesesToken,
                        '.' => Tokens::DotToken,
                        ';' => Tokens::SemicolonToken,
                        _ => {
                            self.reposts.push(Repost {
                                level: Level::Error,
                                error_type: ZXError::SyntaxError,
                                message: "invalid syntax".to_string(),
                                pos: Position {
                                    start: file_stream.index,
                                    end: file_stream.index,
                                },
                            });
                            break;
                        }
                    };

                    self.tokens.push(Token {
                        token_type: kid,
                        pos: Position {
                            start: file_stream.index,
                            end: file_stream.index,
                        },
                    })
                }
                '\r' | ' ' => {}
                _ => {
                    identifier_string.push(currently);
                }
            }

            file_stream.next()
        }

        let filter_reposts: Vec<&Repost> = self.reposts
            .iter()
            .filter(|repost| {
                match repost.level {
                    Level::Error => true,
                    _ => false
                }
            })
            .collect();

        for report in self.reposts.iter() {
            report.print();
        }

        if filter_reposts.is_empty() {
            self.tokens.push(Token {
                token_type: Tokens::EOF,
                pos: Position {
                    start: file_stream.index,
                    end: file_stream.index,
                },
            });

            Ok(())
        } else {
            Err(())
        }
    }

    fn lex_string(&mut self, string_stream: &mut StringStream) -> Result<(), ()> {
        let mut string_content = String::new();
        let mut end_double_quotes = false;
        let start = string_stream.index.clone();
        string_stream.next();

        while !string_stream.is_eof {
            match string_stream.get_currently() {
                '"' => {
                    end_double_quotes = true;
                    break;
                }
                '\\' => {
                    string_stream.next();

                    if !string_stream.is_eof {
                        string_content.push(string_stream.get_currently());
                    } else {
                        break;
                    }
                }
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

    fn lex_slash(&mut self, string_stream: &mut StringStream) -> Result<(), ()> {
        let start = string_stream.index;
        string_stream.next();

        match string_stream.get_currently() {
            '/' => {
                while !string_stream.is_eof && string_stream.get_currently() != '\n' {
                    string_stream.next();
                }
                Ok(())
            }
            '*' => {
                let mut end_comment = false;

                while !string_stream.is_eof {
                    if string_stream.get_currently() == '*' {
                        string_stream.next();
                        if string_stream.get_currently() == '/' {
                            end_comment = true;
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
                    self.push_syntax_error("invalid syntax", Position {
                        start,
                        end: start + 1,
                    });
                    Err(())
                }
            }
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

    fn push_syntax_error(&mut self, error_message: &str, pos: Position) {
        self.reposts.push(Repost {
            level: Level::Error,
            error_type: ZXError::SyntaxError,
            message: String::from(error_message),
            pos,
        });
    }
}
