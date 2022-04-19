mod syntax;
mod test;

use std::slice::Iter;
use util::ast::Statement;
use util::error::ZXError;
use util::repost::{Level, Report};
use util::token::Token;
use util::token::Tokens;

pub struct Parser<'a> {
    pub tokens: Iter<'a, Token>,
    pub index: usize,
    currently: &'a Token,
    is_eof: bool,
    pub asts: Vec<Statement>,
    reposts: Vec<Report>,
}

impl Parser<'_> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        let mut tokens_iter = tokens.iter();
        let next_token = tokens_iter.next().unwrap();

        Parser {
            tokens: tokens_iter,
            index: 0,
            is_eof: next_token.is_token_type(&Tokens::EOF),
            currently: next_token,
            asts: vec![],
            reposts: vec![],
        }
    }

    pub fn comparison(&mut self, token: &Tokens) -> Result<Token, ZXError> {
        if self.currently.is_token_type(token) {
            let ret_token = self.currently.clone();
            self.next_token(if let Tokens::LineSeparatorToken = token { true } else { false });
            Ok(ret_token)
        } else {
            Err(ZXError::SyntaxError {
                message: format!(
                    "Unexpected token {}, expected token {}",
                    self.currently.token_type.to_string(),
                    token.to_string()
                ),
                pos: self.currently.pos.clone(),
            })
        }
    }

    pub fn comparison_string(&mut self, tokens: Vec<&str>) -> Result<Token, ZXError> {
        for token in &tokens {
            if self.currently.is_token_type_str(token) {
                let ret_token = self.currently.clone();
                self.next_token(*token == "LineSeparatorToken");
                return Ok(ret_token);
            }
        }

        Err(ZXError::SyntaxError {
            message: format!(
                "Unexpected token {}, expected token {}",
                self.currently.token_type.to_string(),
                tokens.iter().map(|&x| x).collect::<Vec<&str>>().join(", ")
            ),
            pos: self.currently.pos.clone(),
        })
    }

    pub fn next_token(&mut self, line_separator_token: bool) {
        let token = loop {
            let token = self.tokens.next();

            if let Some(content) = token {
                if !content.is_token_type(&Tokens::LineSeparatorToken) || line_separator_token {
                    break token;
                }
            } else {
                break token;
            }
        };

        self.is_eof = token.is_none();

        if !self.is_eof {
            self.currently = token.unwrap();
            if !token.unwrap().is_token_type(&Tokens::EOF) {
                self.index += 1;
            } else {
                self.is_eof = true;
            }
        }
    }

    fn add_error(&mut self, error: ZXError) {
        self.reposts.push(Report {
            level: Level::Error,
            error: error,
        });
    }

    pub fn parse(&mut self, path: &String, source: &String) {
        while !self.is_eof {
            let statement = self.statement();

            if let Ok(statement) = statement {
                self.asts.push(statement);
            } else if let Err(error) = statement {
                self.add_error(error);
                break
            }
        }

        for repost in &self.reposts {
            repost.print(source, path);
        }
    }
}
