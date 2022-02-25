mod block_syntax;
mod function_syntax;

use crate::Parser;
use util::ast::{Expression, Statement};
use util::error::ZXError;
use util::token::Tokens;

impl Parser<'_> {
    pub fn statement(&mut self) -> Result<Statement, ZXError> {
        let keyword = self.currently;

        if let Tokens::IdentifierToken { ref literal } = keyword.token_type {
            let statement = match literal.as_str() {
                "fn" => Some(self.function_syntax()?),
                _ => None,
            };
            if !statement.is_none() {
                return Ok(statement.unwrap());
            }
        } else if let Tokens::LeftCurlyBracketsToken = keyword.token_type {
            return Ok(self.block_syntax()?);
        }

        Err(ZXError::SyntaxError {
            message: "without this keyword".to_string(),
            pos: keyword.clone().pos,
        })
    }

    // pub fn expressions(&self) -> Result<Expression, ZXError> {
    //
    // }
}
