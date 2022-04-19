use crate::Parser;
use util::ast::{Expression, Parameter, Statement};
use util::error::ZXError;
use util::token::{Position, Tokens};

impl Parser<'_> {
    pub fn function_syntax(&mut self) -> Result<Statement, ZXError> {
        let fn_keyword = self.comparison_string(vec!["IdentifierToken"])?;
        let function_name = self.comparison_string(vec!["IdentifierToken"])?;
        let left_parentheses = self.comparison(&Tokens::LeftParenthesesToken)?;
        let parameters = self.parameters_parse(left_parentheses.pos.clone())?;
        let right_parentheses = self.comparison(&Tokens::RightParenthesesToken)?;
        let type_expression = match self.currently.token_type {
            Tokens::ColonToken => Some(self.type_syntax()?),
            _ => None
        };
        let block = self.block_syntax()?;

        Ok(Statement::FunctionDeclaration {
            fn_keyword,
            function_name,
            left_parentheses,
            parameters,
            right_parentheses,
            return_type: type_expression,
            block: Box::new(block),
        })
    }

    fn parameters_parse(&mut self, left: Position) -> Result<Vec<Parameter>, ZXError> {
        let mut parameters: Vec<Parameter> = vec![];
        let mut comma = true;
        let mut is_close = false;

        // parse parameters
        while !self.is_eof {
            match &self.currently.token_type {
                Tokens::RightParenthesesToken => {
                    is_close = true;
                    break;
                }
                Tokens::IdentifierToken { literal: _ } => {
                    if comma {
                        comma = false;
                        let parameter_name = self.comparison_string(vec!["IdentifierToken"])?;
                        let type_expression = self.type_syntax()?;

                        parameters.push(Parameter {
                            parameter_name,
                            type_expression,
                        });
                    } else {
                        let pos = if let Expression::Type {
                            nullable,
                            ref identifier
                        } = parameters.last().unwrap().type_expression {
                            let pos = if nullable {
                                identifier.pos.start + 1
                            } else {
                                identifier.pos.start
                            };
                            Some(Position {
                                start: pos,
                                end: pos,
                            })
                        } else {
                            None
                        };
                        return Err(ZXError::SyntaxError {
                            message: "expected `,`".to_string(),
                            pos: pos.unwrap_or(self.currently.pos.clone()),
                        });
                    }
                }
                Tokens::CommaToken => {
                    self.comparison(&Tokens::CommaToken)?;
                    comma = true;
                }
                _ => break
                //return Err(ZXError::SyntaxError {
                //                     message: format!("expected parameter name, found `{:?}`", self.currently.token_type),
                //                     pos: left,
                //                 })
            }
        }

        if !is_close {
            Err(ZXError::SyntaxError {
                message: "unclosed parenthese".to_string(),
                pos: left,
            })
        } else {
            Ok(parameters)
        }
    }
}
