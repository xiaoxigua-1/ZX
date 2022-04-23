use util::ast::Statement;
use util::error::ZXError;
use crate::Parser;
use crate::syntax::util::set_error_message;

impl Parser<'_> {
    pub fn while_syntax(&mut self) -> Result<Statement, ZXError> {
        let while_keyword = self.comparison_string(vec!["IdentifierToken"])?;

        let condition = set_error_message(
            self.expressions(0),
            String::from("missing condition"),
            &while_keyword.pos
        )?;
        let block = self.block_syntax()?;

        Ok(Statement::WhileLoop {
            while_keyword,
            condition,
            block: Box::new(block)
        })
    }
}