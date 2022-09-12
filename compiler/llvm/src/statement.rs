use util::ast::{Parameter, Statement, Statement::*, Expression};
use crate::Builder;

impl Builder<'_> {
    pub fn statement(&self, statement: &Statement) {
        match statement {
            FunctionDeclaration { function_name, parameters, return_type, block, .. } =>
                self.build_function(function_name.get_string().unwrap(), parameters, return_type, block),
            _ => {}
        }
    }

    pub fn build_function(&self, name: String, parameters: &Vec<Parameter>, return_type: &Option<Expression>, block: &Box<Statement>) {
        if let Some(return_type) = return_type {

        } else {
            self.context.void_type().fn_type(&[], false)
        }
        self.module.add_function(
            name
        )
    }
}