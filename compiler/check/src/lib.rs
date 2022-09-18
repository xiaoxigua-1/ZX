mod checks;
mod test;
mod r#type;

use util::ast::Expression::*;
use util::ast::Statement::*;
use util::ast::{Expression, Statement};
use util::error::ZXError;
use util::report::Level::Error;
use util::report::{Level, Report};
use util::scope::{Scope, ScopeType, Scopes};
use util::token::Tokens::IdentifierToken;
use util::token::{Literal, Position, Token};
use util::zx_type::ZXTyped;

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

    pub fn check(&mut self) -> Scopes {
        let mut scopes = Scopes::new();
        for statement in self.ast.clone() {
            match self.declaration(statement, &mut scopes) {
                Ok(declaration) => scopes.add_scope(declaration),
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
        });

        scopes
    }

    fn statement(
        &mut self,
        statement: Statement,
        scopes: &mut Scopes,
        children: &mut Scopes,
    ) -> Result<(ZXTyped, Option<Position>), ZXError> {
        match statement {
            Return {
                return_expression, ..
            } => {
                let ret_type = self.statement(*return_expression, scopes, children)?;

                Ok(ret_type)
            }
            Statement::Expression { expression } => Ok(self.auto_type(scopes, None, expression)?),
            _ => {
                let scope = self.declaration(statement, scopes)?;
                children.add_scope(scope.clone());
                Ok((ZXTyped::Void, None))
            }
        }
    }

    fn auto_type(
        &self,
        global_scopes: &mut Scopes,
        sub_scopes: Option<&mut Scopes>,
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
                let scope = self.find_scope(
                    if let Some(scopes) = sub_scopes {
                        scopes
                    } else {
                        global_scopes
                    },
                    &call_name,
                )?;

                match scope.scope_type {
                    ScopeType::DefFunction {
                        parameters,
                        return_type,
                        ..
                    } => {
                        if arguments.len() == parameters.len() {
                            for index in 0..arguments.len() {
                                let arg_scope =
                                    self.auto_type(global_scopes, None, arguments[index].clone())?;
                                let parameter_type = if let ScopeType::DefVariable { var_type } =
                                    &parameters[index].scope_type
                                {
                                    Ok(var_type)
                                } else {
                                    Err(ZXError::InternalError { message: "".into() })
                                }?;

                                if !arg_scope.0.eq(parameter_type) {
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
                                let scope = match self.find_scope_str(
                                    global_scopes,
                                    &return_type.to_string(),
                                    call_name.pos.clone(),
                                ) {
                                    Ok(scope) => Ok(scope),
                                    _ => Err(ZXError::TypeError { message: format!(
                                        "member reference base type '{}' is not a structure or union",
                                        return_type.to_string()
                                    ), pos: call_name.pos.clone() })
                                }?;

                                if let ScopeType::DefClass { members } = scope.scope_type {
                                    self.auto_type(
                                        global_scopes,
                                        Some(&mut members.clone()),
                                        *next,
                                    )?
                                    .0
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
                            let ret = self
                                .auto_type(global_scopes, Some(&mut members.clone()), *next)?
                                .0;
                            ret
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
                                let scope = self.find_scope(global_scopes, &identifier)?;

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
            SubMember { sub_member } => self.auto_type(global_scopes, sub_scopes, *sub_member),
            Identifier { identifier, next } => {
                let scope = self.find_scope(global_scopes, &identifier)?;

                match scope.scope_type {
                    ScopeType::DefVariable { var_type } => {
                        let var_type = if let Some(next) = next {
                            let scope = self.find_scope_str(
                                global_scopes,
                                &var_type.to_string(),
                                identifier.pos.clone(),
                            )?;

                            if let ScopeType::DefClass { .. } = scope.scope_type {
                                self.auto_type(global_scopes, sub_scopes, *next)?.0
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

    fn find_scope(&self, scopes: &mut Scopes, name: &Token) -> Result<Scope, ZXError> {
        self.find_scope_str(scopes, &name.get_string()?, name.pos.clone())
    }

    fn find_scope_str(
        &self,
        scopes: &mut Scopes,
        name: &String,
        pos: Position,
    ) -> Result<Scope, ZXError> {
        if let Some(find) = scopes.find_scope(name) {
            Ok(find)
        } else {
            Err(ZXError::NameError {
                message: format!("NameError: name '{}' is not defined", name),
                pos,
            })
        }
    }
}
