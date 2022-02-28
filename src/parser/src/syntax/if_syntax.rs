use util::ast::Statement;
use util::error::ZXError;
use util::token::{Tokens};
use crate::Parser;

impl Parser<'_> {
    pub fn if_syntax(&mut self) -> Result<Statement, ZXError> {
        let if_keyword = self.comparison_string(vec!["IdentifierToken"])?;
        let condition = match self.expressions() {
            Err(_) => {
                return Err(ZXError::SyntaxError {
                    message: "missing condition".to_string(),
                    pos: if_keyword.pos
                })
            }
            Ok(condition) => condition
        };
        let block = self.block_syntax()?;

        let else_statement = match &self.currently.token_type {
            Tokens::IdentifierToken { literal } => {
                if literal.eq("else") {
                    Some(self.else_syntax()?)
                } else {
                    None
                }
            }
            _ => None
        };

        Ok(Statement::If {
            if_keyword,
            condition,
            block: Box::new(block),
            else_statement: Box::new(else_statement)
        })
    }

    pub fn else_syntax(&mut self) -> Result<Statement, ZXError> {
        let else_keyword = self.comparison_string(vec!["IdentifierToken"])?;

        let next = match &self.currently.token_type {
            Tokens::IdentifierToken { literal } => {
                if literal.eq("if") {
                    Some(self.if_syntax()?)
                } else {
                    None
                }
            }
            _ => Some(self.block_syntax()?)
        };

        Ok(Statement::Else {
            else_keyword,
            next: Box::new(next)
        })
    }
}