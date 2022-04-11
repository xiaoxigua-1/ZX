mod test;
mod r#type;

use util::ast::{Parameter, Statement, Expression};
use util::ast::Statement::*;
use util::ast::Expression::*;
use util::error::ZXError;
use util::repost::Repost;
use util::token::Literal;
use crate::r#type::ZXTyped;

enum ScopeType {
    DefFunction { parameters: Vec<Parameter> },
    DefVariable,
    DefClass,
}

struct Scope {
    name: String,
    scope_type: ScopeType,
    uses_num: i32,
    block: Statement,
}

struct File {
    name: String,
    path: String,
    ast: Vec<Statement>,
}

pub struct Checker {
    ast: Vec<Statement>,
    files: Vec<File>,
    scope: Vec<Scope>,
    // types: Vec<Type>,
    reposts: Vec<Repost>,
}

impl Checker {
    pub fn new(ast: Vec<Statement>) -> Checker {
        Checker {
            ast,
            files: vec![],
            scope: vec![],
            // types: vec![],
            reposts: vec![],
        }
    }

    fn check(&mut self) {
        for statement in self.ast.clone() {
            if let Some(scope) = self.statement(statement, &self.scope) {
                self.scope.push(scope);
            } else {

            }
        }
    }

    fn statement(&self, statement: Statement, scope: &Vec<Scope>) -> Option<Scope> {
        match statement {
            FunctionDeclaration { function_name, parameters, block, return_type, .. } => {
                Some(Scope {
                    name: function_name.token_type.to_string(),
                    scope_type: ScopeType::DefFunction {
                        parameters,
                    },
                    uses_num: 0,
                    block: *block,
                })
            }
            _ => None
        }
    }

    fn expression(&self, expression: Expression, return_type: Option<Expression>) {
        match expression {
            Return { return_expression, .. } => {

            }
        }
    }

    fn auto_type(&self, expression: Expression) -> Result<ZXTyped, ZXError> {
        match expression {
            Value { kid, .. } => {
                Ok(match kid {
                    Literal::String => ZXTyped::String,
                    Literal::Char => ZXTyped::Char,
                    Literal::Integer => ZXTyped::Integer,
                    Literal::Float => ZXTyped::Float
                })
            },
            Call { call_name, next, .. } => {},
            Path { identifier, next } => {},
            SubMember { sub_member } => {},
            _ => {}
        }
    }
}