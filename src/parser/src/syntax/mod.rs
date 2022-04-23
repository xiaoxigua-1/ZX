mod block_syntax;
mod function_syntax;
mod type_syntax;
mod return_syntax;
mod variable_declaration_syntax;
mod if_syntax;
mod while_syntax;
mod for_loop_syntax;
mod util;

use crate::Parser;
use ::util::ast::{Expression, Statement};
use ::util::error::ZXError;
use ::util::token::{Token, Tokens};
use crate::syntax::util::{infix_binding_power, is_operator, operator_type};

impl Parser<'_> {
    pub fn statement(&mut self) -> Result<Statement, ZXError> {
        let keyword = self.currently;

        return if let Tokens::IdentifierToken { ref literal } = keyword.token_type {
            let statement = match literal.as_str() {
                "fn" => self.function_syntax()?,
                "pub" => {
                    self.comparison_string(vec!["IdentifierToken"])?;
                    Statement::Public {
                        statement: Box::new(self.statement()?)
                    }
                }
                "static" => {
                    self.comparison_string(vec!["IdentifierToken"])?;
                    Statement::Static {
                        statement: Box::new(self.statement()?)
                    }
                }
                "return" => self.return_syntax()?,
                "var" => self.variable_declaration_syntax()?,
                "if" => self.if_syntax()?,
                "while" => self.while_syntax()?,
                "for" => self.for_syntax()?,
                _ => Statement::Expression { expression: self.expressions(0)? },
            };

            Ok(statement)
        } else if let Tokens::LeftCurlyBracketsToken = keyword.token_type {
            Ok(self.block_syntax()?)
        } else {
            Ok(Statement::Expression { expression: self.expressions(0)? })
        };
    }

    pub fn expressions(&mut self, min_bp: u8) -> Result<Expression, ZXError> {
        match &self.currently.token_type {
            Tokens::LiteralToken { kid, literal: _ } => {
                let content = self.comparison_string(vec!["LiteralToken"])?;
                match &self.currently.token_type {
                    Tokens::DotToken => {
                        Ok(Expression::Value {
                            kid: kid.clone(),
                            content,
                            next: Box::new(Some(self.expressions(min_bp)?)),
                        })
                    },
                    token_type if is_operator(token_type) => {
                        Ok(self.operator_expression(min_bp, Expression::Value {
                            kid: kid.clone(),
                            content,
                            next: Box::new(None),
                        })?)
                    }
                    _ => {
                        Ok(Expression::Value {
                            kid: kid.clone(),
                            content,
                            next: Box::new(None),
                        })
                    }
                }
            }
            Tokens::IdentifierToken { .. } | Tokens::StdToken => {
                let token = self.comparison_string(vec!["IdentifierToken", "StdToken"])?;

                let expression = match self.currently.token_type {
                    // call expression
                    Tokens::LeftParenthesesToken => self.call_expression(token)?,
                    _ => {
                        let next = match &self.currently.token_type {
                            Tokens::DotToken | Tokens::ColonToken | Tokens::LeftParenthesesToken => Some(Box::new(self.expressions(min_bp)?)),
                            _ => None
                        };

                        match &token.token_type {
                            Tokens::IdentifierToken { ref literal } if literal == "true" || literal == "false" => Expression::Bool {
                                identifier: token,
                            },
                            _ => Expression::Identifier {
                                identifier: token,
                                next,
                            }
                        }
                    }
                };

                Ok(expression)
            }
            Tokens::DotToken => {
                self.comparison(&Tokens::DotToken)?;
                let sub_member = Box::new(self.expressions(min_bp)?);

                Ok(Expression::SubMember {
                    sub_member
                })
            }
            Tokens::ColonToken => {
                self.comparison(&Tokens::ColonToken)?;
                self.comparison(&Tokens::ColonToken)?;

                let expression = self.expressions(min_bp)?;

                Ok(Expression::Path {
                    next: Box::new(expression),
                })
            }
            Tokens::LeftParenthesesToken => Ok(self.operator_brackets()?),
            _ => Err(ZXError::SyntaxError {
                message: "Unknown Token".to_string(),
                pos: self.currently.pos.clone(),
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
                Tokens::CommaToken => {
                    self.comparison(&Tokens::CommaToken)?;
                    comma = true
                }
                _ => {
                    if comma {
                        arguments.push(self.expressions(0)?);
                        comma = false;
                    } else {
                        return Err(ZXError::SyntaxError {
                            message: "abc".to_string(),
                            pos: self.currently.pos.clone(),
                        });
                    }
                }
            }
        }

        let right_parentheses = self.comparison(&Tokens::RightParenthesesToken)?;

        let next = match self.currently.token_type {
            Tokens::ColonToken | Tokens::DotToken => {
                Some(Box::new(self.expressions(0)?))
            }
            _ => None
        };

        Ok(Expression::Call {
            call_name,
            left_parentheses,
            arguments,
            right_parentheses,
            next,
        })
    }

    fn operator_expression(&mut self, min_bp: u8, left: Expression) -> Result<Expression, ZXError> {
        let mut left_expression = left;

        loop {
            let operator = match &self.currently.token_type {
                token_type if is_operator(token_type) => operator_type(&self.currently)?,
                _ => break
            };

            let bp = infix_binding_power(&operator);

            if bp < min_bp {
                break;
            }

            self.next(false);
            let next = self.expressions(min_bp + 1)?;
            let right_expression = self.operator_expression(bp + 1, next)?;

            left_expression = Expression::Operator {
                operator_type: operator,
                left: Box::new(left_expression),
                right: Box::new(right_expression),
            }
        }

        Ok(left_expression)
    }

    fn operator_brackets(&mut self) -> Result<Expression, ZXError> {
        self.comparison(&Tokens::LeftParenthesesToken)?;
        let next = self.expressions(0)?;
        let operator = self.operator_expression(0, next)?;
        self.comparison(&Tokens::RightParenthesesToken)?;

        Ok(Expression::Brackets {
            content: Box::new(operator)
        })
    }
}
