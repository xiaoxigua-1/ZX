mod file_stream;
mod test;
mod lex;
mod escapes;

use std::fs;
use file_stream::StringStream;
use util::repost::{Level, Repost};
use util::error::ZXError;
use util::token::{Position, Token, Tokens};

pub struct Lexer {
    path: String,
    pub tokens: Vec<Token>,
    reposts: Vec<Repost>,
}

impl Lexer {
    pub fn new(path: &String) -> Lexer {
        Lexer {
            path: path.to_string(),
            tokens: vec![],
            reposts: vec![],
        }
    }

    pub fn lexer(&mut self) -> Result<(), ()> {
        let file_string = fs::read_to_string(self.path.clone())
            .expect("Something went wrong reading the file");
        let mut file_stream = StringStream::new(&file_string);
        let mut identifier_string = String::new();

        while !file_stream.is_eof {
            let currently = file_stream.get_currently();

            match currently {
                ' '..='/' | ':'..='@' | '['..='^' | '{'..='~' | '\n' | '\r' | '`' => {
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
                '"' | '/' | '\'' => {
                    let result = match currently {
                        '"' => self.lex_string(&mut file_stream),
                        '/' => self.lex_slash(&mut file_stream),
                        '\'' => self.lex_char(&mut file_stream),
                        _ => Result::Ok(())
                    };

                    match result {
                        Ok(()) => {}
                        Err(()) => {
                            break;
                        }
                    }
                }
                '!'..='.' | ':'..='@' | '['..='^' | '{'..='~' | '\n' | '`' => {
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
                        ':' => Tokens::ColonToken,
                        '!' => Tokens::ExclamationToken,
                        _ => {
                            self.push_syntax_error("invalid syntax", Position {
                                start: file_stream.index,
                                end: file_stream.index,
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
                    });
                }
                '\r' | ' ' => {}
                _ => {
                    identifier_string.push(currently);
                }
            }

            file_stream.next();
        }

        let is_to_eof = self.reposts
            .iter()
            .filter(|repost| {
                match repost.level {
                    Level::Error => true,
                    _ => false
                }
            })
            .collect::<Vec<&Repost>>()
            .is_empty();

        for report in self.reposts.iter() {
            report.print(&file_string, &self.path);
        }

        if is_to_eof {
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

    fn push_syntax_error(&mut self, error_message: &str, pos: Position) {
        self.reposts.push(Repost {
            level: Level::Error,
            error_type: ZXError::SyntaxError,
            message: String::from(error_message),
            pos,
        });
    }
}
