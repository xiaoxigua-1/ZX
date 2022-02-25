use crate::Parser;
use util::ast::Statement;
use util::error::ZXError;
use util::token::Tokens;

impl Parser<'_> {
    pub fn block_syntax(&mut self) -> Result<Statement, ZXError> {
        let left_curly_brackets = self.comparison(&Tokens::LeftCurlyBracketsToken)?;
        let mut statements = vec![];

        loop {
            match self.currently {
                token if token.is_token_type(&Tokens::RightCurlyBracketsToken) => break,
                _ if self.is_eof => {
                    return Err(ZXError::SyntaxError {
                        message: "unclosed delimiter".to_string(),
                        pos: left_curly_brackets.pos,
                    });
                }
                _ => statements.push(self.statement()?),
            }
        }

        let right_curly_brackets = self.comparison(&Tokens::RightCurlyBracketsToken)?;

        Ok(Statement::Block {
            left_curly_brackets,
            statements,
            right_curly_brackets,
        })
    }
}
