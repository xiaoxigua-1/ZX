use inkwell::values::FunctionValue;
use util::ast::Statement;

use crate::Builder;

impl Builder<'_> {
    pub fn statements(&self, statement: &Statement, function: FunctionValue) {
        match &statement {
            Statement::Block { statements, .. } => self.block_statement(statements, function),
            _ => {}
        }
    }

    pub fn block_statement(&self, statements: &Vec<Statement>, function: FunctionValue) {
        statements.iter().for_each(|statement| {
            self.statements(statement, function);
        });
    }
}