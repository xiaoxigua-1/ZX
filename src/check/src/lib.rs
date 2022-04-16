mod test;
mod r#type;
mod scope;

use util::ast::{Parameter, Statement, Expression};
use util::ast::Statement::*;
use util::ast::Expression::*;
use util::error::ZXError;
use util::repost::{Level, Repost};
use util::token::{Literal, Tokens};
use crate::r#type::ZXTyped;
use crate::scope::{Scope, Scopes, ScopeType};

struct File {
    name: String,
    path: String,
    ast: Vec<Statement>,
}

pub struct Checker {
    ast: Vec<Statement>,
    files: Vec<File>,
    scopes: Scopes,
    types: Vec<ZXTyped>,
    reposts: Vec<Repost>,
}

impl Checker {
    pub fn new(ast: Vec<Statement>) -> Checker {
        Checker {
            ast,
            files: vec![],
            scopes: Scopes::new(),
            types: vec![],
            reposts: vec![],
        }
    }

    pub fn check(&mut self) {
        for statement in self.ast.clone() {
            match self.statement(statement, vec![&self.scopes]) {
                Ok(scope) => {
                    self.scopes.add_scope(scope);
                },
                Err(error) => {
                    self.reposts.push(Repost {
                        level: Level::Error,
                        error
                    })
                }
            }
        }

        println!("{:?}", self.scopes);
    }

    fn statement(&self, statement: Statement, scopes: Vec<&Scopes>) -> Result<Scope, ZXError> {
        match statement {
            FunctionDeclaration { function_name, parameters, block, return_type, .. } => {
                // TODO: Check function block and parameters type and return type
                Ok(Scope {
                    name: if let Tokens::IdentifierToken { literal } = function_name.token_type {
                        literal
                    } else {
                        return Err(ZXError::UnknownError { message: "".to_string() })
                    },
                    scope_type: ScopeType::DefFunction {
                        parameters,
                        return_type: if let Some(expression) = return_type {
                            self.auto_type(expression)?
                        } else {
                            ZXTyped::Void
                        }
                    },
                    uses_num: 0,
                    block: *block,
                })
            }
            _ => Err(ZXError::UnknownError { message: String::from("") })
        }
    }

    // fn expression(&self, expression: Expression, return_type: Option<Expression>) {
    //     match expression {
    //         Return { return_expression, .. } => {
    //
    //         }
    //     }
    // }

    fn auto_type(&self, expression: Expression) -> Result<ZXTyped, ZXError> {
        match expression {
            Value { kid, .. } => {
                // value type
                Ok(match kid {
                    Literal::String => ZXTyped::String,
                    Literal::Char => ZXTyped::Char,
                    Literal::Integer => ZXTyped::Integer,
                    Literal::Float => ZXTyped::Float
                })
            },
            Call { call_name, next, .. } => {
                // TODO: return type
                Ok(ZXTyped::Other {
                    type_name: call_name,
                })
            },
            // Path { identifier, next } => {
            //     // TODO: path end type
            // },
            // SubMember { sub_member } => {
            //     // TODO: sub　member type
            // },
            _ => Err(ZXError::UnknownError { message: String::from("")})
        }
    }

    fn find_scope(&self, scopes: Vec<Scopes>, name: &String) -> Option<Scope> {
        let mut find_scope = None;
        for scope in scopes.iter() {
            if let Some(find) = scope.find_scope(name) {
                find_scope = Some(find);
                break;
            }
        }

        find_scope
    }
}