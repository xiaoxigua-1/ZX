use util::ast::Statement;
use util::error::ZXError;
use crate::Parser;

impl Parser<'_>{
    fn for_syntax(&mut self) -> Result<Statement, ZXError>{
        let for_keyword = self.comparcomparison_string(vec!["IdentifierToken"]);
        let condition = match self.expressions(){
            Err(_) =>{
                ZXError::SyntaxError{
                    message: "missing condition".to_string(),
                    pos: for_keyword.pos,
                }
            }
            OK(condition)=> condition
        };
        let block = self.blocksyntax;
        Ok(Statement::ForLoop {
            for_keyword,
            condition,
            block: Box::new(block)
        })
    }
}