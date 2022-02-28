use util::ast::Statement;
use util::error::ZXError;
use crate::Parser;

impl Parser<'_> {
    pub fn while_syntax(&mut self) -> Result<Statement, ZXError> {
        let while_keyword = self.comparison_string(vec!["IdentifierToken"])?;
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

        Ok(Statement::WhileLoop {
            while_keyword,
            condition,
            block: Box::new(block)
        })
    }
}