mod file_stream;
pub mod token;
mod test;

use std::fs;
use file_stream::StringStream;
use util::repost::{Level, Repost};
use crate::token::{Literal, Position, Token, Tokens};

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
            match file_stream.get_currently() {
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

            match file_stream.get_currently() {
                '"' => {
                    match self.lex_string(&mut file_stream) {
                        Err(()) => {
                            self.reposts.push(Repost {
                                level: Level::Error,
                                message: "EOL while scanning string literal",
                            });

                            break;
                        }
                        Ok(token) => {
                            self.tokens.push(token);
                        }
                    }
                }
                '*' => {
                    self.tokens.push(Token {
                        token_type: Tokens::MultiplyToken,
                        pos: Position {
                            start: file_stream.index,
                            end: file_stream.index,
                        },
                    });
                }
                '\n' => {
                    self.tokens.push(Token {
                        token_type: Tokens::LineSeparator,
                        pos: Position {
                            start: file_stream.index,
                            end: file_stream.index,
                        },
                    });
                }
                '\r' => {}
                _ => {
                    identifier_string.push(file_stream.get_currently());
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

    fn lex_string(&self, string_stream: &mut StringStream) -> Result<Token, ()> {
        let mut string_content = String::new();
        let start = string_stream.index.clone();
        string_stream.next();

        while !string_stream.is_eof {
            match string_stream.get_currently() {
                '"' => {
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

        if !string_stream.is_eof {
            Ok(Token {
                token_type: Tokens::LiteralToken {
                    kid: Literal::String,
                    literal: string_content,
                },
                pos: Position {
                    start,
                    end: string_stream.index
                }
            })
        } else {
            Err(())
        }
    }
}
