mod scope;
mod test;
mod r#type;

use crate::r#type::ZXTyped;
use crate::scope::{Scope, ScopeType, Scopes};
use util::ast::Expression::*;
use util::ast::Statement::*;
use util::ast::{Expression, Statement};
use util::error::ZXError;
use util::report::Level::Error;
use util::report::{Level, Report};
use util::token::Tokens::IdentifierToken;
use util::token::{Literal, Position, Token};

struct File {
    name: String,
    path: String,
    ast: Vec<Statement>,
}

pub struct Checker {
    ast: Vec<Statement>,
    files: Vec<File>,
    pub reposts: Vec<Report>,
}

impl Checker {
    pub fn new(ast: Vec<Statement>) -> Checker {
        Checker {
            ast,
            files: vec![],
            reposts: vec![],
        }
    }

    pub fn check(&mut self) {
        let mut scopes = vec![Scopes::new()];
        for statement in self.ast.clone() {
            if let Err(error) = self.declaration(statement, &mut scopes) {
                self.reposts.push(Report {
                    level: Error,
                    error,
                })
            }
        }

        self.reposts.push(Report {
            level: Level::Debug,
            error: ZXError::Debug {
                message: format!("scopes {:#?}", scopes),
            },
        })
    }

    fn declaration(
        &mut self,
        statement: Statement,
        scopes: &mut Vec<Scopes>,
    ) -> Result<(), ZXError> {
        match statement {
            FunctionDeclaration {
                function_name,
                parameters,
                block,
                return_type,
                ..
            } => {
                // TODO: Check function block and parameters type and return type
                let return_type = if let Some(expression) = return_type {
                    self.auto_type(scopes, expression)?
                } else {
                    (ZXTyped::Void, None)
                };

                if let Some(scope) = scopes.last_mut() {
                    scope.add_scope(Scope {
                        name: if let IdentifierToken { literal } = function_name.token_type {
                            literal
                        } else {
                            return Err(ZXError::UnknownError {
                                message: "".to_string(),
                            });
                        },
                        scope_type: ScopeType::DefFunction {
                            parameters,
                            block: *block.clone(),
                            return_type: return_type.0.clone(),
                        },
                        uses_num: 0,
                        pos: function_name.pos,
                    });
                }

                match self.statement(*block, scopes) {
                    Err(error) => self.reposts.push(Report {
                        level: Error,
                        error,
                    }),
                    Ok(ret_type) => {
                        if return_type.0 != ret_type.0 {
                            return Err(ZXError::TypeError {
                                message: "mismatched types".to_string(),
                                pos: ret_type.1.unwrap(),
                            });
                        }
                    }
                }
            }
            VariableDeclaration {
                var_name,
                type_identifier,
                value,
                equal,
                ..
            } => {
                let auto_type = if let Some(type_expression) = type_identifier {
                    let auto_type = self.auto_type(scopes, type_expression)?;

                    if let Some(value) = value {
                        if let Statement::Expression { expression } = *value {
                            let value_type = self.auto_type(scopes, expression.clone())?;
                            if auto_type.0 != value_type.0 {
                                return Err(ZXError::TypeError {
                                    message: "mismatched types".to_string(),
                                    pos: auto_type.1.unwrap(),
                                });
                            }
                        }
                    }

                    auto_type
                } else {
                    if let Some(value) = value {
                        if let Statement::Expression { expression } = *value {
                            self.auto_type(scopes, expression.clone())?
                        } else {
                            return Err(ZXError::SyntaxError {
                                message: "this is not a expression".to_string(),
                                pos: equal.unwrap().pos,
                            });
                        }
                    } else {
                        return Err(ZXError::TypeError {
                            message: "type annotations needed".to_string(),
                            pos: var_name.pos.clone(),
                        });
                    }
                };

                if let Some(scope) = scopes.last_mut() {
                    scope.add_scope(Scope {
                        name: if let IdentifierToken { literal } = var_name.token_type {
                            literal
                        } else {
                            return Err(ZXError::UnknownError {
                                message: "".to_string(),
                            });
                        },
                        scope_type: ScopeType::DefVariable {
                            var_type: auto_type.0,
                        },
                        uses_num: 0,
                        pos: var_name.pos,
                    })
                }
            }
            _ => {
                return Err(ZXError::UnknownError {
                    message: String::from("Unknown statement."),
                });
            }
        }

        Ok(())
    }

    fn statement(
        &mut self,
        statement: Statement,
        scopes: &mut Vec<Scopes>,
    ) -> Result<(ZXTyped, Option<Position>), ZXError> {
        match statement {
            Block {
                statements,
                left_curly_brackets,
                ..
            } => {
                let mut ret = (ZXTyped::Void, Some(left_curly_brackets.pos));
                scopes.push(Scopes::new());
                for statement in statements.iter() {
                    match self.statement(statement.clone(), scopes) {
                        Ok(ret_type) => ret = ret_type,
                        Err(error) => self.reposts.push(Report {
                            level: Error,
                            error,
                        }),
                    }
                }
                if let Some(last_scope) = scopes.last() {
                    let no_used_list = last_scope.no_used_variables_or_functions();
                    no_used_list.iter().for_each(|no_used_scope| {
                        self.reposts.push(Report {
                            level: Level::Warning,
                            error: ZXError::Warning {
                                message: format!("field is never read: `{}`", no_used_scope.name),
                                pos: no_used_scope.pos.clone(),
                            },
                        })
                    })
                }
                scopes.pop();
                Ok(ret)
            }
            Return {
                return_expression, ..
            } => {
                let ret_type = self.statement(*return_expression, scopes)?;

                Ok(ret_type)
            }
            Statement::Expression { expression } => Ok(self.auto_type(scopes, expression)?),
            _ => {
                self.declaration(statement, scopes)?;
                Ok((ZXTyped::Void, None))
            }
        }
    }

    fn auto_type(
        &self,
        scopes: &mut Vec<Scopes>,
        expression: Expression,
    ) -> Result<(ZXTyped, Option<Position>), ZXError> {
        match expression {
            Value { kid, content, .. } => {
                // value type
                Ok((
                    match kid {
                        Literal::String => ZXTyped::String { nullable: false },
                        Literal::Char => ZXTyped::Char { nullable: false },
                        Literal::PositiveInteger => ZXTyped::Integer { nullable: false },
                        Literal::Float => ZXTyped::Float { nullable: false },
                        Literal::NegativeInteger => ZXTyped::Integer { nullable: false },
                    },
                    Some(content.pos),
                ))
            }
            Call {
                call_name,
                // next,
                left_parentheses,
                right_parentheses,
                arguments,
                ..
            } => {
                // TODO: return type
                let scope = self.find_scope(scopes, &call_name)?;

                match scope.scope_type {
                    ScopeType::DefFunction {
                        parameters,
                        return_type,
                        ..
                    } => {
                        if arguments.len() == parameters.len() {
                            for index in 0..arguments.len() {
                                let parameter = &parameters[index];
                                let arg_scope = self.auto_type(scopes, arguments[index].clone())?;
                                let parameter_scope =
                                    self.auto_type(scopes, parameter.type_expression.clone())?;

                                if arg_scope.0 != parameter_scope.0 {
                                    return Err(ZXError::TypeError {
                                        message: format!("mismatched types"),
                                        pos: arg_scope.1.unwrap(),
                                    });
                                }
                            }
                        } else {
                            return Err(ZXError::TypeError {
                                message: format!(
                                    "this function takes {} argument but {} arguments were supplied",
                                    parameters.len(),
                                    arguments.len()),
                                pos: Position {
                                    start: left_parentheses.pos.start,
                                    end: right_parentheses.pos.end + 1,
                                },
                            });
                        }
                        Ok((
                            return_type,
                            Option::from(Position {
                                start: call_name.pos.start,
                                end: right_parentheses.pos.end,
                            }),
                        ))
                    }
                    _ => Err(ZXError::NameError {
                        message: format!("NameError: name '{}' is not defined", scope.name),
                        pos: call_name.pos,
                    }),
                }
            }
            Type {
                identifier,
                nullable,
            } => {
                if let IdentifierToken { literal } = &identifier.token_type {
                    Ok((
                        match literal.as_ref() {
                            "Int" => ZXTyped::Integer { nullable },
                            "Float" => ZXTyped::Float { nullable },
                            "Str" => ZXTyped::String { nullable },
                            "Char" => ZXTyped::Char { nullable },
                            "Void" => ZXTyped::Void,
                            _ => {
                                let scope = self.find_scope(scopes, &identifier)?;

                                if let ScopeType::DefClass = &scope.scope_type {
                                    ZXTyped::Other {
                                        type_name: scope.name,
                                    }
                                } else {
                                    return Err(ZXError::TypeError {
                                        message: format!("type `{}` not found", literal),
                                        pos: identifier.pos.clone(),
                                    });
                                }
                            }
                        },
                        Some(identifier.pos),
                    ))
                } else {
                    Err(ZXError::UnknownError {
                        message: "".to_string(),
                    })
                }
            }
            // Path { identifier, next } => {
            // TODO: path end type
            // },
            // SubMember { sub_member } => {
            // TODO: sub　member type
            // },
            Identifier { identifier, .. } => {
                let scope = self.find_scope(scopes, &identifier)?;

                match scope.scope_type {
                    ScopeType::DefVariable { var_type } => Ok((var_type, Some(identifier.pos))),
                    ScopeType::DefFunction { .. } => Err(ZXError::TypeError {
                        message: format!("`{}` is Function not a variable", scope.name),
                        pos: identifier.pos,
                    }),
                    _ => Err(ZXError::UnknownError {
                        message: "".to_string(),
                    }),
                }
            }
            _ => Err(ZXError::UnknownError {
                message: "123".to_string(),
            }),
        }
    }

    fn find_scope(&self, scopes: &mut Vec<Scopes>, name: &Token) -> Result<Scope, ZXError> {
        if let IdentifierToken { literal } = &name.token_type {
            for scope in scopes.iter_mut() {
                if let Some(find) = scope.find_scope(literal) {
                    return Ok(find);
                }
            }

            Err(ZXError::NameError {
                message: format!("NameError: name '{}' is not defined", literal),
                pos: name.pos.clone(),
            })
        } else {
            Err(ZXError::UnknownError {
                message: "".to_string(),
            })
        }
    }
}
