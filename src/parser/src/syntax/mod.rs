mod block_syntax;
mod function_syntax;
mod type_syntax;

use crate::Parser;
use util::ast::{Expression, Statement};
use util::error::ZXError;
use util::token::{Token, Tokens};

impl Parser<'_> {
    pub fn statement(&mut self) -> Result<Statement, ZXError> {
        let keyword = self.currently;

        return if let Tokens::IdentifierToken { ref literal } = keyword.token_type {
            let statement = match literal.as_str() {
                "fn" => self.function_syntax()?,
                _ => Statement::Expression { expression: self.expressions()? },
            };
            Ok(statement)
        } else if let Tokens::LeftCurlyBracketsToken = keyword.token_type {
             Ok(self.block_syntax()?)
        } else {
            Ok(Statement::Expression { expression: self.expressions()? })
        };
        //
        // Err(ZXError::SyntaxError {
        //     message: "without this keyword".to_string(),
        //     pos: keyword.clone().pos,
        // })
    }

    pub fn expressions(&mut self) -> Result<Expression, ZXError> {
        match &self.currently.token_type {
            Tokens::LiteralToken { kid, literal: _ } => {
                let content = self.comparison_string(vec!["LiteralToken"])?;

                Ok(Expression::Value {
                    kid: kid.clone(),
                    content
                })
            }
            Tokens::IdentifierToken { .. } => {
                let token = self.comparison_string(vec!["IdentifierToken"])?;

                let expression = match self.currently.token_type {
                    Tokens::ColonToken => {
                        self.comparison(&Tokens::ColonToken)?;
                        self.comparison(&Tokens::ColonToken)?;

                        let expression = self.expressions()?;

                        Expression::Path {
                            identifier: token,
                            next: Box::new(expression)
                        }
                    }
                    // call expression
                    Tokens::LeftParenthesesToken => self.call_expression(token)?,
                    _ => Expression::Identifier {
                        identifier: token
                    }
                };

                Ok(expression)
            }
            _ => Err(ZXError::SyntaxError {
                message: "".to_string(),
                pos: self.currently.pos.clone()
            })
        }
    }

    fn call_expression(&mut self, call_name: Token) -> Result<Expression, ZXError> {
        let left_parentheses = self.comparison(&Tokens::LeftParenthesesToken)?;
        let mut comma = true;
        let mut arguments: Vec<Expression> = vec![];

        loop {
            match self.currently.token_type {
                Tokens::RightParenthesesToken => break,
                Tokens::CommaToken => comma = true,
                _ => {
                    if comma {
                        arguments.push(self.expressions()?);
                        comma = false;
                    } else {
                        return Err(ZXError::SyntaxError {
                            message: "".to_string(),
                            pos: self.currently.pos.clone()
                        })
                    }
                }
            }
        }

        let right_parentheses = self.comparison(&Tokens::RightParenthesesToken)?;

        let next =  match self.currently.token_type {
            Tokens::ColonToken => {
                self.comparison(&Tokens::ColonToken)?;
                self.comparison(&Tokens::ColonToken)?;
                Some(self.expressions()?)
            }
            _ => None
        };

        Ok(Expression::Call {
            call_name,
            left_parentheses,
            arguments,
            right_parentheses,
            next: Box::new(next)
        })
    }
}
