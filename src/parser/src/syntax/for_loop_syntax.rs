use crate::Parser;
use util::ast::Statement;
use util::error::ZXError;

impl Parser<'_> {
    pub(crate) fn for_syntax(&mut self) -> Result<Statement, ZXError> {
        let for_keyword = self.comparison_string(vec!["IdentifierToken"])?;
        let for_var_name = self.comparison_string(vec!["IdentifierToken"])?;
        let for_in_keyword = self.comparison_string(vec!["IdentifierToken"])?;
        let iter = Box::new(self.statement()?);
        let block = self.block_syntax()?;

        Ok(Statement::ForLoop {
            for_keyword,
            for_var_name,
            for_in_keyword,
            iter,
            block: Box::new(block),
        })
    }
}
