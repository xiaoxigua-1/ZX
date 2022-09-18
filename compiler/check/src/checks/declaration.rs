use crate::ScopeType::DefClass;
use crate::{Checker, ZXTyped};
use util::ast::Statement;
use util::ast::Statement::{Block, Class, FunctionDeclaration, VariableDeclaration};
use util::error::ZXError;
use util::report::Level::{self, Error};
use util::report::Report;
use util::scope::{Scope, ScopeType, Scopes};
use util::token::Position;
use util::token::Tokens::IdentifierToken;

impl Checker {
    pub fn declaration(
        &mut self,
        statement: Statement,
        scopes: &mut Scopes,
    ) -> Result<Scope, ZXError> {
        match statement {
            FunctionDeclaration {
                function_name,
                parameters,
                block,
                return_type,
                ..
            } => {
                let return_type = if let Some(expression) = return_type {
                    self.auto_type(scopes, None, expression)?
                } else {
                    (ZXTyped::Void, None)
                };
                
                let parameters = parameters
                    .iter()
                    .map(|parameter| {
                        let scope = self.auto_type(scopes, None, parameter.type_expression.clone())?;
                        Ok(Scope {
                            name: parameter.parameter_name.get_string()?,
                            pos: parameter.parameter_name.pos.clone(),
                            scope_type: ScopeType::DefVariable { var_type: scope.0 },
                            uses_num: 0,
                        })
                    }).collect::<Result<Vec<Scope>, ZXError>>()?;

                let scope = Scope {
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
                };

                match self.declaration(*block, scopes) {
                    Err(error) => self.reposts.push(Report {
                        level: Error,
                        error,
                    }),
                    Ok(scope) => {
                        if let ScopeType::Block { ret, .. } = scope.scope_type {
                            if return_type.0 != ret.0 {
                                return Err(ZXError::TypeError {
                                    message: "mismatched types".to_string(),
                                    pos: ret.1.unwrap(),
                                });
                            }
                        }
                    }
                }

                Ok(scope)
            }
            VariableDeclaration {
                var_name,
                type_identifier,
                value,
                equal,
                ..
            } => {
                let auto_type = if let Some(type_expression) = type_identifier {
                    let auto_type = self.auto_type(scopes, None, type_expression)?;

                    if let Some(value) = value {
                        if let Statement::Expression { expression } = *value {
                            let value_type = self.auto_type(scopes, None, expression.clone())?;
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
                            self.auto_type(scopes, None, expression.clone())?
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

                Ok(Scope {
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
            Class {
                class_name, member, ..
            } => {
                let mut members = Scopes::new();

                for member in member {
                    members.add_scope(self.declaration(member, scopes)?);
                }
                Ok(Scope {
                    name: if let IdentifierToken { literal } = class_name.token_type {
                        literal
                    } else {
                        return Err(ZXError::UnknownError {
                            message: "".to_string(),
                        });
                    },
                    pos: class_name.pos,
                    scope_type: DefClass { members },
                    uses_num: 0,
                })
            }
            Block {
                left_curly_brackets,
                statements,
                right_curly_brackets,
            } => {
                let mut children = Scopes::new();
                let mut ret = (ZXTyped::Void, Some(right_curly_brackets.pos.clone()));
                for statement in statements.iter() {
                    match self.statement(statement.clone(), scopes, &mut children) {
                        Ok(ret_type) => {
                            if ret_type.1.is_some() {
                                ret = ret_type
                            }
                        }
                        Err(error) => self.reposts.push(Report {
                            level: Error,
                            error,
                        }),
                    }
                }

                children
                    .no_used_variables_or_functions()
                    .iter()
                    .for_each(|no_used_scope| {
                        self.reposts.push(Report {
                            level: Level::Warning,
                            error: ZXError::Warning {
                                message: format!("field is never read: `{}`", no_used_scope.name),
                                pos: no_used_scope.pos.clone(),
                            },
                        })
                    });

                Ok(Scope {
                    name: "_".into(),
                    scope_type: ScopeType::Block { children, ret },
                    uses_num: 0,
                    pos: Position {
                        start: left_curly_brackets.pos.start.clone(),
                        end: right_curly_brackets.pos.end.clone(),
                    },
                })
            }
            _ => {
                return Err(ZXError::UnknownError {
                    message: String::from("Unknown statement."),
                });
            }
        }
    }
}
