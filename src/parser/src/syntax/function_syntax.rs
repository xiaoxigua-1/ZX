use crate::Parser;
use util::ast::Statement;
use util::error::ZXError;
use util::token::Tokens;

impl Parser<'_> {
    pub fn function_syntax(&mut self) -> Result<Statement, ZXError> {
        let fn_keyword = self.comparison_string(vec!["IdentifierToken"])?;
        let function_name = self.comparison_string(vec!["IdentifierToken"])?;
        let left_parentheses = self.comparison(&Tokens::LeftParenthesesToken)?;
        // parse parameters
        let right_parentheses = self.comparison(&Tokens::RightParenthesesToken)?;
        let type_expression = match self.currently.token_type {
            Tokens::ColonToken => Some(self.type_syntax()?),
            _ => None
        };
        let block = self.block_syntax()?;

        Ok(Statement::FunctionDeclaration {
            fn_keyword,
            function_name,
            left_parentheses,
            parameters: vec![],
            right_parentheses,
            return_type: type_expression,
            block: Box::new(block),
        })
    }

    fn parameters_parse(&mut self) {
    }
}
