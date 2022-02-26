mod block_syntax;
mod function_syntax;
mod type_syntax;

use crate::Parser;
use util::ast::{Expression, Statement};
use util::error::ZXError;
use util::token::Tokens;

impl Parser<'_> {
    pub fn statement(&mut self) -> Result<Statement, ZXError> {
        let keyword = self.currently;

        if let Tokens::IdentifierToken { ref literal } = keyword.token_type {
            let statement = match literal.as_str() {
                "fn" => Some(self.function_syntax()?),
                _ => None,
            };
            if !statement.is_none() {
                return Ok(statement.unwrap());
            }
        } else if let Tokens::LeftCurlyBracketsToken = keyword.token_type {
            return Ok(self.block_syntax()?);
        } else {
            return Ok(Statement::Expression { expression: self.expressions()? });
        }

        Err(ZXError::SyntaxError {
            message: "without this keyword".to_string(),
            pos: keyword.clone().pos,
        })
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

                match self.currently.token_type {
                    Tokens::ColonToken => {
                        self.comparison(&Tokens::ColonToken)?;
                        self.comparison(&Tokens::ColonToken)?;
                        let expression = self.expressions()?;

                        Ok(Expression::Path {
                            identifier: token,
                            next: Box::new(expression)
                        })
                    }
                    // call expression
                    Tokens::LeftParenthesesToken => {
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
                            call_name: token,
                            left_parentheses,
                            arguments,
                            right_parentheses,
                            next: Box::new(next)
                        })
                    }
                    _ => Ok(Expression::Identifier {
                        identifier: token
                    })
                }
            }
            _ => Err(ZXError::SyntaxError {
                message: "".to_string(),
                pos: self.currently.pos.clone()
            })
        }
    }
}
