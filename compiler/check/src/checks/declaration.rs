use crate::ScopeType::DefClass;
use crate::{Checker, ZXTyped};
use util::ast::Statement;
use util::ast::Statement::{Block, Class, FunctionDeclaration, VariableDeclaration};
use util::bytecode::BytecodeType;
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
        path: String,
        children: &mut Vec<Scopes>,
    ) -> Result<Scope, ZXError> {
        match statement {
            FunctionDeclaration {
                function_name,
                parameters,
                block,
                return_type,
                ..
            } => {
                let mut param_index = 0;
                let return_type = if let Some(expression) = return_type {
                    let ret = self.auto_type(scopes, None, children, expression)?;
                    (ret.0, ret.1)
                } else {
                    (ZXTyped::Void, None)
                };

                let parameters = parameters
                    .iter()
                    .map(|parameter| {
                        let scope = self.auto_type(
                            scopes,
                            None,
                            children,
                            parameter.type_expression.clone(),
                        )?;
                        let name = parameter.parameter_name.get_string()?;
                        let path = format!("{}${}", path, name);
                        let scope = Scope {
                            name,
                            path,
                            pos: parameter.parameter_name.pos.clone(),
                            scope_type: ScopeType::DefVariable {
                                var_type: scope.0,
                                value: Some(BytecodeType::param_value(param_index)),
                            },
                            uses_num: 0,
                        };
                        param_index = param_index + 1;
                        Ok(scope)
                    })
                    .collect::<Result<Vec<Scope>, ZXError>>()?;
                let name = if let IdentifierToken { literal } = function_name.token_type {
                    literal
                } else {
                    return Err(ZXError::UnknownError {
                        message: "".to_string(),
                    });
                };
                let path = format!("{}${}", path, name);
                let scope = Scope {
                    name: name.clone(),
                    path: path.clone(),
                    scope_type: ScopeType::DefFunction {
                        parameters: parameters.clone(),
                        block: BytecodeType::Box { bytecodes: vec![] },
                        return_type: return_type.0.clone(),
                        children: Scopes::new(),
                    },
                    uses_num: 0,
                    pos: function_name.pos.clone(),
                };
                children.last_mut().unwrap().add_scope(scope.clone());
                let block_scope = self.declaration(*block, scopes, path.clone(), children)?;
                let block = if let ScopeType::Block {
                    ret,
                    bytecodes,
                    children,
                } = block_scope.scope_type
                {
                    if return_type.0 != ret.0 {
                        Err(ZXError::TypeError {
                            message: "mismatched types".to_string(),
                            pos: return_type.1.unwrap(),
                        })
                    } else {
                        Ok((BytecodeType::Box { bytecodes }, children))
                    }
                } else {
                    Err(ZXError::InternalError { message: "".into() })
                }?;

                Ok(Scope {
                    name,
                    path,
                    scope_type: ScopeType::DefFunction {
                        parameters,
                        block: block.0,
                        return_type: return_type.0.clone(),
                        children: block.1,
                    },
                    uses_num: 0,
                    pos: function_name.pos,
                })
            }
            VariableDeclaration {
                var_name,
                type_identifier,
                value,
                equal,
                ..
            } => {
                let auto_type = if let Some(type_expression) = type_identifier {
                    let auto_type = self.auto_type(scopes, None, children, type_expression)?;

                    if let Some(value) = value {
                        if let Statement::Expression { expression } = *value {
                            let value_type =
                                self.auto_type(scopes, None, children, expression.clone())?;
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
                            self.auto_type(scopes, None, children, expression.clone())?
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
                let name = if let IdentifierToken { literal } = var_name.token_type {
                    literal
                } else {
                    return Err(ZXError::UnknownError {
                        message: "".to_string(),
                    });
                };
                let path = format!("{}${}", path, name);
                Ok(Scope {
                    name,
                    path,
                    scope_type: ScopeType::DefVariable {
                        var_type: auto_type.0,
                        value: auto_type.2,
                    },
                    uses_num: 0,
                    pos: var_name.pos,
                })
            }
            Class {
                class_name, member, ..
            } => {
                let mut members = Scopes::new();
                let name = if let IdentifierToken { literal } = class_name.token_type {
                    literal
                } else {
                    return Err(ZXError::UnknownError {
                        message: "".to_string(),
                    });
                };
                let path = format!("{}${}", path, name);

                for member in member {
                    members.add_scope(self.declaration(member, scopes, path.clone(), children)?);
                }
                Ok(Scope {
                    name,
                    path,
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
                let mut ret_type = (ZXTyped::Void, Some(right_curly_brackets.pos.clone()));
                let mut bytecodes: Vec<BytecodeType> = vec![];
                children.push(Scopes::new());
                for statement in statements.iter() {
                    match self.statement(statement.clone(), scopes, children, path.clone()) {
                        Ok(ret) => {
                            if ret.1.is_some() {
                                ret_type = (ret.0, ret.1);
                                if ret.2.is_some() {
                                    bytecodes.push(ret.2.unwrap())
                                }
                            }
                        }
                        Err(error) => self.reposts.push(Report {
                            level: Error,
                            error,
                        }),
                    }
                }
                let children_clone = children.last().unwrap().clone();
                children_clone
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
                children.pop();
                Ok(Scope {
                    name: "$".into(),
                    path,
                    scope_type: ScopeType::Block {
                        children: children_clone,
                        ret: ret_type,
                        bytecodes,
                    },
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
