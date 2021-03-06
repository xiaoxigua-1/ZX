mod checks;
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

pub struct Checker {
    ast: Vec<Statement>,
    pub reposts: Vec<Report>,
}

impl Checker {
    pub fn new(ast: Vec<Statement>) -> Checker {
        Checker {
            ast,
            reposts: vec![],
        }
    }

    pub fn check(&mut self) {
        let mut scopes = vec![Scopes::new()];
        for statement in self.ast.clone() {
            match self.declaration(statement, &mut scopes) {
                Ok(declaration) => scopes.last_mut().unwrap().add_scope(declaration),
                Err(error) => self.reposts.push(Report {
                    level: Error,
                    error,
                }),
            }
        }

        self.reposts.push(Report {
            level: Level::Debug,
            error: ZXError::Debug {
                message: format!("scopes {:#?}", scopes),
            },
        })
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
                let scope = self.declaration(statement, scopes)?;
                scopes.last_mut().unwrap().add_scope(scope);
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
                next,
                left_parentheses,
                right_parentheses,
                arguments,
                ..
            } => {
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
                            if let Some(next) = next {
                                let scope = self.find_scope_str(
                                    scopes,
                                    &return_type.to_string(),
                                    call_name.pos.clone(),
                                )?;

                                if let ScopeType::DefClass { members } = scope.scope_type {
                                    let mut scopes = scopes.clone();
                                    scopes.push(members);
                                    self.auto_type(&mut scopes, *next)?.0
                                } else {
                                    return Err(ZXError::UnknownError {
                                        message: String::new(),
                                    });
                                }
                            } else {
                                return_type
                            },
                            Option::from(Position {
                                start: call_name.pos.start,
                                end: right_parentheses.pos.end,
                            }),
                        ))
                    }
                    ScopeType::DefClass { members } => {
                        let return_type = if let Some(next) = next {
                            let mut scopes = scopes.clone();
                            scopes.push(members);

                            self.auto_type(&mut scopes, *next)?.0
                        } else {
                            ZXTyped::Other(scope.name)
                        };
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

                                if let ScopeType::DefClass { .. } = &scope.scope_type {
                                    ZXTyped::Other(scope.name)
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
            SubMember { sub_member } => self.auto_type(scopes, *sub_member),
            Identifier { identifier, next } => {
                let scope = self.find_scope(scopes, &identifier)?;

                match scope.scope_type {
                    ScopeType::DefVariable { var_type } => {
                        let var_type = if let Some(next) = next {
                            let scope = self.find_scope_str(
                                scopes,
                                &var_type.to_string(),
                                identifier.pos.clone(),
                            )?;

                            if let ScopeType::DefClass { members } = scope.scope_type {
                                let mut scopes = scopes.clone();
                                scopes.push(members);
                                self.auto_type(&mut scopes, *next)?.0
                            } else {
                                return Err(ZXError::UnknownError {
                                    message: String::new(),
                                });
                            }
                        } else {
                            var_type
                        };
                        Ok((var_type, Some(identifier.pos)))
                    }
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
            self.find_scope_str(scopes, literal, name.pos.clone())
        } else {
            Err(ZXError::UnknownError {
                message: "".to_string(),
            })
        }
    }

    fn find_scope_str(
        &self,
        scopes: &mut Vec<Scopes>,
        name: &String,
        pos: Position,
    ) -> Result<Scope, ZXError> {
        for scope in scopes.iter_mut().rev() {
            if let Some(find) = scope.find_scope(name) {
                return Ok(find);
            }
        }

        Err(ZXError::NameError {
            message: format!("NameError: name '{}' is not defined", name),
            pos,
        })
    }

    fn next_scope(&self) {}
}
