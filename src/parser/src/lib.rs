mod test;
mod syntax;

use util::token::Token;
use util::token::Tokens;
use std::slice::Iter;
use util::error::ZXError;

pub struct Parser<'a> {
    pub tokens: Iter<'a, Token>,
    pub index: usize,
    currently: &'a Token,
    is_eof: bool,
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
        }
    }

    pub fn comparison(&mut self, token: &Tokens) -> Result<Token, ZXError> {
        if self.currently.is_token_type(token) {
            let ret_token = self.currently.clone();
            self.next_token();
            Ok(ret_token)
        } else {
            Err(ZXError::SyntaxError {
                message: format!("Unexpected token {}, expected token {}", self.currently.token_type.to_string(), token.to_string()),
                pos: self.currently.pos.clone(),
            })
        }
    }

    pub fn comparison_string(&self, token: &str) -> Result<Token, ZXError> {
        if self.currently.is_token_type_str(token) {
            Ok(self.currently.clone())
        } else {
            Err(ZXError::SyntaxError {
                message: format!("Unexpected token {}, expected token {}", self.currently.token_type.to_string(), token.to_string()),
                pos: self.currently.pos.clone(),
            })
        }
    }

    pub fn next_token(&mut self) {
        let token = self.tokens.next();
        self.is_eof = token.is_none();

        if !self.is_eof {
            if token.unwrap().is_token_type(&Tokens::EOF) {
                self.currently = token.unwrap();
                self.index += 1;
            }
        }
    }

    pub fn parse(&mut self) {
        self.statement();
    }
}