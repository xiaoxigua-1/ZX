use util::ast::Statement;
use util::ast::Statement::{FunctionDeclaration, VariableDeclaration};
use util::error::ZXError;
use util::report::Level::Error;
use util::report::Report;
use util::token::Tokens::IdentifierToken;
use crate::{Checker, Scope, Scopes, ScopeType, ZXTyped};

impl Checker {
    pub fn declaration(
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
}