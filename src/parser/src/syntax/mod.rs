mod function_syntax;

use crate::Parser;
use util::error::ZXError;
use util::ast::{Expression, Statement};
use util::token::Tokens;

impl Parser<'_> {
    pub fn statement(&mut self) -> Result<Statement, ZXError> {
        let keyword = self.comparison_string(vec!["IdentifierToken"])?;

        if let Tokens::IdentifierToken { ref literal } = keyword.token_type {
            let statement = match literal.as_str() {
                "fn" => Some(self.function_syntax(keyword.clone())?),
                _ => None
            };
            if !statement.is_none() {
                return Ok(statement.unwrap());
            }
        }

        Err(ZXError::SyntaxError {
            message: "without this keyword".to_string(),
            pos: keyword.pos
        })
    }

    // pub fn expressions(&self) -> Result<Expression, ZXError> {
    //
    // }
}