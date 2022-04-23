use crate::Parser;
use util::ast::Statement;
use util::error::ZXError;

impl Parser<'_> {
    pub fn return_syntax(&mut self) -> Result<Statement, ZXError> {
        let return_keyword = self.comparison_string(vec!["IdentifierToken"])?;
        let return_expression = self.statement()?;

        Ok(Statement::Return {
            return_keyword,
            return_expression: Box::new(return_expression),
        })
    }
}
