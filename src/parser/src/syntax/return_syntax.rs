use util::ast::Statement;
use util::error::ZXError;
use crate::Parser;

impl Parser<'_> {
    pub fn return_syntax(&mut self) -> Result<Statement, ZXError> {
        let return_keyword = self.comparison_string(vec!["IdentifierToken"])?;
        let return_expression = self.expressions()?;

        Ok(Statement::Return {
            return_keyword,
            return_expression,
        })
    }
}