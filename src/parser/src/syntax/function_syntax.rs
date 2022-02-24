use util::ast::Statement;
use util::error::ZXError;
use util::token::{Token, Tokens};
use crate::Parser;

impl Parser<'_> {
    pub fn function_syntax(&mut self, fn_keyword: Token) -> Result<Statement, ZXError> {
        let function_name = self.comparison_string(vec!["IdentifierToken"])?;
        let left_parentheses = self.comparison(&Tokens::LeftParenthesesToken)?;
        //
        let right_parentheses = self.comparison(&Tokens::RightParenthesesToken)?;
        let left_curly_brackets = self.comparison(&Tokens::LeftCurlyBracketsToken)?;
        let mut statements: Vec<Statement> = vec![];

        loop {
            match self.currently {
                token if token.is_token_type(&Tokens::RightCurlyBracketsToken) => break,
                token if self.is_eof => {
                    return Err(ZXError::SyntaxError {
                        message: "unclosed delimiter".to_string(),
                        pos: left_curly_brackets.pos
                    });
                }
                _ => statements.push(self.statement()?)
            }
        }

        let right_curly_brackets = self.comparison(&Tokens::RightCurlyBracketsToken)?;

        Ok(Statement::FunctionDeclaration {
            fn_keyword,
            function_name,
            left_parentheses,
            parameters: vec![],
            right_parentheses,
            left_curly_brackets,
            statement: statements,
            right_curly_brackets
        })
    }
}
