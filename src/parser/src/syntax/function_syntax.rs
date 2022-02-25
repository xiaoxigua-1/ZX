use util::ast::Statement;
use util::error::ZXError;
use util::token::{Tokens};
use crate::Parser;

impl Parser<'_> {
    pub fn function_syntax(&mut self) -> Result<Statement, ZXError> {
        let fn_keyword = self.comparison_string(vec!["IdentifierToken"])?;
        let function_name = self.comparison_string(vec!["IdentifierToken"])?;
        let left_parentheses = self.comparison(&Tokens::LeftParenthesesToken)?;
        // parse parameters
        let right_parentheses = self.comparison(&Tokens::RightParenthesesToken)?;
        let block = self.block_syntax()?;


        Ok(Statement::FunctionDeclaration {
            fn_keyword,
            function_name,
            left_parentheses,
            parameters: vec![],
            right_parentheses,
            block: Box::new(block),
        })
    }
}
