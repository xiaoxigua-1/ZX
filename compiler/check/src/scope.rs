use crate::ZXTyped;
use util::ast::{Parameter, Statement};

#[derive(Clone, Debug)]
pub enum ScopeType {
    DefFunction {
        parameters: Vec<Parameter>,
        block: Statement,
        return_type: ZXTyped,
    },
    DefVariable {
        var_type: ZXTyped,
    },
    DefClass,
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub(crate) name: String,
    pub(crate) scope_type: ScopeType,
    pub(crate) uses_num: i32,
}

#[derive(Debug)]
pub struct Scopes {
    scopes: Vec<Scope>,
}

impl Scopes {
    pub fn new() -> Scopes {
        Scopes { scopes: vec![] }
    }

    pub fn find_scope(&self, name: &String) -> Option<Scope> {
        if let Some(find) = self.scopes.iter().find(|scope| scope.name.eq(name)) {
            Some(find.clone())
        } else {
            None
        }
    }

    pub fn add_scope(&mut self, scope: Scope) {
        self.scopes.push(scope);
    }
}
