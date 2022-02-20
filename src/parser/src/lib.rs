mod test;

use util::token::Token;
use util::token::Tokens;
use std::slice::Iter;

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
            is_eof: next_token.is_token_type(Tokens::EOF),
            currently: next_token,
        }
    }

    pub fn next_token(&mut self) {
        let token = self.tokens.next();
        self.is_eof = token.is_none();

        if !self.is_eof {
            if token.unwrap().is_token_type(Tokens::EOF) {
                self.currently = token.unwrap();
                self.index += 1;
            }
        }
    }

    pub fn parse(&mut self) {
        while !self.is_eof {
            if self.currently.is_token_type_str("LiteralToken") {
                if let Tokens::LiteralToken { kid, literal } = &self.currently.token_type {
                    println!("{:?}: {}", kid, literal);
                };

            }
            self.next_token();
        }
    }
}