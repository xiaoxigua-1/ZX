mod scope;
mod test;
mod r#type;

use crate::r#type::ZXTyped;
use crate::scope::{Scope, ScopeType, Scopes};
use util::ast::Expression::*;
use util::ast::Statement::*;
use util::ast::{Expression, Parameter, Statement};
use util::error::ZXError;
use util::report::{Level, Report};
use util::token::{Literal, Tokens};
use util::token::Tokens::IdentifierToken;

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
        for statement in self.ast.clone() {
            match self.statement(statement, vec![&self.global_scopes]) {
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

    fn statement(&self, statement: Statement, scopes: Vec<&Scopes>) -> Result<Scope, ZXError> {
        match statement {
            FunctionDeclaration {
                function_name,
                parameters,
                block,
                return_type,
                ..
            } => {
                // TODO: Check function block and parameters type and return type
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
                            self.find_scope(scopes, &expression)?
                        } else {
                            ZXTyped::Void
                        },
                    },
                    uses_num: 0,
                })
            },
            VariableDeclaration { var_name, type_identifier, value, .. } => Ok(Scope {
                name: if let IdentifierToken { literal } = var_name.token_type {
                    literal
                } else {
                    return Err(ZXError::UnknownError {
                        message: "".to_string(),
                    });
                },
                scope_type: ScopeType::DefVariable {
                    var_type: ZXTyped::String
                },
                uses_num: 0,
            }),
            _ => Err(ZXError::UnknownError {
                message: String::from("Unknown statement."),
            }),
        }
    }

    // fn expression(&self, expression: Expression, return_type: Option<Expression>) {
    //     match expression {
    //         Return { return_expression, .. } => {
    //
    //         }
    //     }
    // }

    fn auto_type(&self, expression: Expression) -> Option<ZXTyped> {
        match expression {
            Value { kid, .. } => {
                // value type
                Some(match kid {
                    Literal::String => ZXTyped::String,
                    Literal::Char => ZXTyped::Char,
                    Literal::PositiveInteger => ZXTyped::Integer,
                    Literal::Float => ZXTyped::Float,
                    Literal::NegativeInteger => ZXTyped::Integer,
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
            Type { identifier, nullable } => {
                if let IdentifierToken { literal } = identifier.token_type {
                    Some(match literal.as_ref() {
                        "Int" => ZXTyped::Integer,
                        "Float" => ZXTyped::Float,
                        "Str" => ZXTyped::String,
                        "Char" => ZXTyped::Char,
                        _ => ZXTyped::Other { type_name: literal }
                    })
                } else {
                    None
                }
            }
            // Path { identifier, next } => {
            // TODO: path end type
            // },
            // SubMember { sub_member } => {
            // TODO: sub　member type
            // },
            _ => None,
        }
    }

    fn find_scope(&self, scopes: Vec<&Scopes>, expression: &Expression) -> Result<ZXTyped, ZXError> {
        let identifier = if let Type { identifier, .. } = expression {
            Some(identifier)
        } else {
            None
        };
        let literal = if let IdentifierToken { literal } = &identifier.unwrap().token_type {
            Some(literal)
        } else {
            None
        };
        let mut find_scope = None;
        for scope in scopes.iter() {
            if let Some(find) = scope.find_scope(literal.unwrap()) {
                find_scope = Some(find.name);
                break;
            }
        }
        if let Some(find_scope) = find_scope {
            Ok(ZXTyped::Other {
                type_name: find_scope
            })
        } else {
            Err(ZXError::TypeError { message: format!("type `{}` not found", literal.unwrap()), pos: identifier.unwrap().pos.clone() })
        }
    }
}
