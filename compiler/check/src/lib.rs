mod scope;
mod test;
mod r#type;

use crate::r#type::ZXTyped;
use crate::scope::{Scope, ScopeType, Scopes};
use util::ast::Expression::*;
use util::ast::Statement::*;
use util::ast::{Expression, Statement};
use util::error::ZXError;
use util::report::{Level, Report};
use util::token::Tokens::IdentifierToken;
use util::token::{Literal, Token};

struct File {
    name: String,
    path: String,
    ast: Vec<Statement>,
}

pub struct Checker {
    ast: Vec<Statement>,
    files: Vec<File>,
    global_scopes: Scopes,
    types: Vec<ZXTyped>,
    pub reposts: Vec<Report>,
}

impl Checker {
    pub fn new(ast: Vec<Statement>) -> Checker {
        Checker {
            ast,
            files: vec![],
            global_scopes: Scopes::new(),
            types: vec![],
            reposts: vec![],
        }
    }

    pub fn check(&mut self) {
        let scopes = self.global_scopes.clone();
        for statement in self.ast.clone() {
            match self.declaration(statement, vec![&scopes]) {
                Ok(scope) => {
                    self.global_scopes.add_scope(scope);
                }
                Err(error) => self.reposts.push(Report {
                    level: Level::Error,
                    error,
                }),
            }
        }

        self.reposts.push(Report {
            level: Level::Debug,
            error: ZXError::Debug {
                message: format!("global {:#?}", self.global_scopes),
            },
        })
    }

    fn declaration(&mut self, statement: Statement, scopes: Vec<&Scopes>) -> Result<Scope, ZXError> {
        match statement {
            FunctionDeclaration {
                function_name,
                parameters,
                block,
                return_type,
                ..
            } => {
                // TODO: Check function block and parameters type and return type
                if let Err(error) = self.statement(*block.clone(), scopes.clone()) {
                    self.reposts.push(Report {
                        level: Level::Error,
                        error,
                    });
                };
                Ok(Scope {
                    name: if let IdentifierToken { literal } = function_name.token_type {
                        literal
                    } else {
                        return Err(ZXError::UnknownError {
                            message: "".to_string(),
                        });
                    },
                    scope_type: ScopeType::DefFunction {
                        parameters,
                        block: *block,
                        return_type: if let Some(expression) = return_type {
                            self.auto_type(scopes, expression)?
                        } else {
                            ZXTyped::Void
                        },
                    },
                    uses_num: 0,
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
                    let auto_type = self.auto_type(scopes.clone(), type_expression)?;

                    if let Some(value) = value {
                        if let Statement::Expression { expression } = *value {
                            let value_type = self.auto_type(scopes, expression.clone())?;
                            if let Type { identifier, .. } = expression {
                                if auto_type != value_type {
                                    return Err(ZXError::TypeError {
                                        message: "mismatched types".to_string(),
                                        pos: identifier.pos,
                                    });
                                }
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
                Ok(Scope {
                    name: if let IdentifierToken { literal } = var_name.token_type {
                        literal
                    } else {
                        return Err(ZXError::UnknownError {
                            message: "".to_string(),
                        });
                    },
                    scope_type: ScopeType::DefVariable {
                        var_type: auto_type,
                    },
                    uses_num: 0,
                })
            }
            _ => Err(ZXError::UnknownError {
                message: String::from("Unknown statement."),
            }),
        }
    }

    fn statement(&mut self, statement: Statement, scopes: Vec<&Scopes>) -> Result<ZXTyped, ZXError> {
        match statement {
            Block { statements, .. } => {
                for statement in statements.iter() {
                    self.statement(statement.clone(), scopes.clone())?;
                }

                Ok(ZXTyped::Void)
            }
            _ => {
                self.declaration(statement, scopes)?;
                Ok(ZXTyped::Void)
            }
        }
    }

    // fn expression(&self, expression: Expression, return_type: Option<Expression>) {
    //     match expression {
    //         Return { return_expression, .. } => {
    //
    //         }
    //     }
    // }

    fn auto_type(&self, scopes: Vec<&Scopes>, expression: Expression) -> Result<ZXTyped, ZXError> {
        match expression {
            Value { kid, .. } => {
                // value type
                Ok(match kid {
                    Literal::String => ZXTyped::String { nullable: false },
                    Literal::Char => ZXTyped::Char { nullable: false },
                    Literal::PositiveInteger => ZXTyped::Integer { nullable: false },
                    Literal::Float => ZXTyped::Float { nullable: false },
                    Literal::NegativeInteger => ZXTyped::Integer { nullable: false },
                })
            }
            // Call {
            //     call_name, next, ..
            // } => {
            //     // TODO: return type
            //     Ok(ZXTyped::Other {
            //         type_name: call_name,
            //     })
            // }
            Type {
                identifier,
                nullable,
            } => {
                if let IdentifierToken { literal } = &identifier.token_type {
                    Ok(match literal.as_ref() {
                        "Int" => ZXTyped::Integer { nullable },
                        "Float" => ZXTyped::Float { nullable },
                        "Str" => ZXTyped::String { nullable },
                        "Char" => ZXTyped::Char { nullable },
                        _ => {
                            let scope = self.find_scope(scopes, &identifier)?;

                            if let ScopeType::DefClass = &scope.scope_type {
                                ZXTyped::Other {
                                    type_name: scope.name,
                                }
                            } else {
                                return Err(ZXError::TypeError {
                                    message: format!("type `{}` not found", literal),
                                    pos: identifier.pos,
                                });
                            }
                        }
                    })
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
            _ => Err(ZXError::UnknownError {
                message: "".to_string(),
            }),
        }
    }

    fn find_scope(&self, scopes: Vec<&Scopes>, name: &Token) -> Result<Scope, ZXError> {
        if let IdentifierToken { literal } = &name.token_type {
            for scope in scopes.iter() {
                if let Some(find) = scope.find_scope(literal) {
                    return Ok(find);
                }
            }

            Err(ZXError::TypeError {
                message: format!("type `{}` not found", literal),
                pos: name.pos.clone(),
            })
        } else {
            Err(ZXError::UnknownError {
                message: "".to_string(),
            })
        }
    }
}
