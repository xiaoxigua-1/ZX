use crate::Parser;
use util::ast::Statement;
use util::error::ZXError;
use util::token::Tokens;

impl Parser<'_> {
    pub fn variable_declaration_syntax(&mut self) -> Result<Statement, ZXError> {
        let var_keyword = self.comparison_string(vec!["IdentifierToken"])?;
        let var_name = self.comparison_string(vec!["IdentifierToken"])?;

        let colon = match &self.currently.token_type {
            Tokens::ColonToken => Some(self.comparison(&Tokens::ColonToken)?),
            _ => None,
        };
        let type_identifier = if colon.is_none() {
            None
        } else {
            Some(self.type_syntax()?)
        };
        let equal = match &self.currently.token_type {
            Tokens::EqualToken => Some(self.comparison(&Tokens::EqualToken)?),
            _ => None,
        };
        let value = if equal.is_none() {
            None
        } else {
            Some(Box::new(self.statement()?))
        };

        Ok(Statement::VariableDeclaration {
            var_keyword,
            var_name,
            colon,
            type_identifier,
            equal,
            value,
        })
    }
}
