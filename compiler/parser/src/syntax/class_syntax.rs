use util::ast::Statement;
use util::error::ZXError;
use util::token::Tokens;
use crate::Parser;

impl Parser<'_> {
    pub fn class_syntax(&mut self) -> Result<Statement, ZXError> {
        let class_keyword = self.comparison_string(vec!["IdentifierToken"])?;
        let class_name = self.comparison_string(vec!["IdentifierToken"])?;
        let left_curly_bracket = self.comparison(&Tokens::LeftCurlyBracketsToken)?;
        let mut member: Vec<Statement> = vec![];

        loop {
            match self.currently {
                token if token.is_token_type(&Tokens::RightCurlyBracketsToken) => break,
                _ if self.is_eof => {
                    return Err(ZXError::SyntaxError {
                        message: "unclosed curly bracket".to_string(),
                        pos: left_curly_bracket.pos,
                    });
                }
                _ => {
                    let statement = self.statement()?;

                    match statement {
                        Statement::Public { .. } | Statement::Static { .. } | Statement::FunctionDeclaration { .. } | Statement::VariableDeclaration { .. } => {
                            member.push(statement);
                        }
                        _ => return Err(ZXError::SyntaxError { pos: left_curly_bracket.pos, message: "unknown statement".to_string() })
                    }
                },
            }
        };

        self.comparison(&Tokens::RightCurlyBracketsToken)?;

        Ok(Statement::Class {
            class_keyword,
            class_name,
            clone: None,
            inherit: None,
            member
        })
    }
}