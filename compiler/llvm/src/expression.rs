use util::ast::Expression;
use util::ast::Expression::Type;
use crate::Builder;

impl Builder<'_> {
    pub fn expression(&self, expression: Expression) {
        match expression {
            _ => {}
        }
    }

    pub fn type_express(&self, expression: Expression) {
        if let Expression::Type { identifier,  nullable } = expression {} else {}
    }
}