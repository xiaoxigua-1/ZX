use crate::syntax::syntax_util::set_error_message;
use crate::Parser;
use util::ast::Statement;
use util::error::ZXError;

impl Parser<'_> {
    pub fn while_syntax(&mut self) -> Result<Statement, ZXError> {
        let while_keyword = self.comparison_string(vec!["IdentifierToken"])?;

        let condition = set_error_message(
            self.expressions(0),
            String::from("missing condition"),
            &while_keyword.pos,
        )?;
        let block = self.block_syntax()?;

        Ok(Statement::WhileLoop {
            while_keyword,
            condition,
            block: Box::new(block),
        })
    }
}
