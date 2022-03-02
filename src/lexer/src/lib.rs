mod escapes;
mod file_stream;
mod lex;
mod test;

use file_stream::StringStream;
use util::error::ZXError;
use crate::lex::is_whitespace;
use util::token::{Position, Token, Tokens};

pub struct Lexer {
    source: String,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: &String) -> Lexer {
        Lexer {
            source: source.to_string(),
            tokens: vec![],
        }
    }

    pub fn lexer(&mut self) -> Result<(), ZXError> {
        let source = self.source.clone();
        let mut file_stream = StringStream::new(&source);

        while !file_stream.is_eof {
            let currently = file_stream.get_currently();

            match currently {
                '"' | '/' | '\'' | '-' | '0'..='9' => {
                    match currently {
                        '"' => self.lex_string(&mut file_stream)?,
                        '/' => self.lex_slash(&mut file_stream)?,
                        '\'' => self.lex_char(&mut file_stream)?,
                        '-' | '0'..='9' => self.lex_number(&mut file_stream)?,
                        _ => {},
                    };
                }
                '!'..='.' | ':'..='@' | '['..='^' | '{'..='~' | '\n' | '`' => {
                    let kid = match currently {
                        '\n' => Tokens::LineSeparatorToken,
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
                        '?' => Tokens::QuestionMarkToken,
                        '&' => Tokens::AmpersandToken,
                        ',' => Tokens::CommaToken,
                        '%' => Tokens::PercentToken,
                        '$' => Tokens::StdToken,
                        _ => {
                            return Err(ZXError::SyntaxError {
                                message: "invalid syntax".to_string(),
                                pos: Position {
                                    start: file_stream.index,
                                    end: file_stream.index
                                }
                            });
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
                c if !is_whitespace(c) => {
                    self.lex_identifier(&mut file_stream)?;
                }
                _ => {}
            }

            file_stream.next();
        }

        self.tokens.push(Token {
            token_type: Tokens::EOF,
            pos: Position {
                start: file_stream.index,
                end: file_stream.index,
            },
        });

        Ok(())
    }
}
