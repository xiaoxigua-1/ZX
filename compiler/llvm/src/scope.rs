use util::scope::{Scope, ScopeType};
use crate::Builder;

impl Builder<'_> {
    pub fn scope(&self, scope: &Scope) {
        match &scope.scope_type {
            ScopeType::DefFunction { parameters, block, return_type } => self.build_function(&scope.name),
            _ => {}
        }
    }

    pub fn build_function(&self, name: &String) {
        let function = self.module.add_function(
            &name,
            self.context.void_type().fn_type(&[], false),
            None,
        );
    }
}