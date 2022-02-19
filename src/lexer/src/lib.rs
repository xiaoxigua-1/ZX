mod file_stream;
pub mod token;
mod test;

use std::fs;
use file_stream::StringStream;
use util::Repost;
use crate::token::{Token, Tokens};

struct Lexer {
    path: String,
    tokens: Vec<Token>,
    repost: Vec<Repost>
}

impl Lexer {
    fn new(path: String) -> Lexer {
        Lexer {
            path,
            tokens: vec![],
            repost: vec![]
        }
    }

    fn lexer(&mut self) {
        let file_string = fs::read_to_string(self.path.clone()).expect("Something went wrong reading the file");
        let mut file_stream = StringStream::new(&file_string);
        let mut identifier_string = String::new();

        while !file_stream.is_eof {
            match file_stream.get_currently() {
                ' '..='/' | ':'..='@' | '['..='`' | '{'..='~' | '\n' | '\r' => {
                    if !identifier_string.is_empty() {
                        println!("{}", identifier_string);
                        self.tokens.push(Token {
                            token_type: Tokens::IdentifierToken,
                            literal: identifier_string,
                        });
                        identifier_string = String::new();
                    }
                }
                _ => {}
            }

            match file_stream.get_currently() {
                '*' => {
                    self.tokens.push(Token {
                        token_type: Tokens::MultiplyToken,
                        literal: "*".to_string(),
                    });
                }
                '\n' => {
                    self.tokens.push(Token {
                        token_type: Tokens::LineSeparator,
                        literal: '\n'.to_string(),
                    });
                }
                '\r' => {}
                _ => {
                    identifier_string.push(file_stream.get_currently());
                }
            }

            file_stream.next()
        }

        self.tokens.push(Token { token_type: Tokens::EOF, literal: "".to_string() })
    }
}
