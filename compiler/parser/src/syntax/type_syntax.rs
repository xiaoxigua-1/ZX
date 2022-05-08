use crate::Parser;
use util::ast::Expression;
use util::error::ZXError;
use util::token::Tokens;

impl Parser<'_> {
    pub fn type_syntax(&mut self) -> Result<Expression, ZXError> {
        self.comparison(&Tokens::ColonToken)?;
        let tpye_identifier = self.comparison_string(vec!["IdentifierToken"])?;

        let question_mark = match self.currently.token_type {
            Tokens::QuestionMarkToken => Some(self.comparison(&Tokens::QuestionMarkToken)?),
            _ => None,
        };

        Ok(Expression::Type {
            identifier: tpye_identifier,
            nullable: !question_mark.is_none(),
        })
    }
}
