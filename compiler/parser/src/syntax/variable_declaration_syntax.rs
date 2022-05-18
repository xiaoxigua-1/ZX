use crate::Parser;
use util::ast::Statement;
use util::error::ZXError;
use util::token::Tokens;

impl Parser<'_> {
    pub fn variable_declaration_syntax(&mut self) -> Result<Statement, ZXError> {
        let var_keyword = self.comparison_string(vec!["IdentifierToken"])?;
        let var_name = self.comparison_string(vec!["IdentifierToken"])?;

        let type_identifier = match &self.currently.token_type {
            Tokens::ColonToken => Some(self.type_syntax()?),
            _ => None,
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
            type_identifier,
            equal,
            value,
        })
    }
}
