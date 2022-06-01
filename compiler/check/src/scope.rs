use crate::ZXTyped;
use util::ast::{Parameter, Statement};
use util::token::Position;

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
    DefClass {
        members: Vec<Scope>
    },
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub(crate) name: String,
    pub(crate) pos: Position,
    pub(crate) scope_type: ScopeType,
    pub(crate) uses_num: i32,
}

#[derive(Debug, Clone)]
pub struct Scopes {
    scopes: Vec<Scope>,
}

impl Scopes {
    pub fn new() -> Scopes {
        Scopes { scopes: vec![] }
    }

    pub fn find_scope(&mut self, name: &String) -> Option<Scope> {
        if let Some(find) = self.scopes.iter_mut().find(|scope| scope.name.eq(name)) {
            find.uses_num += 1;
            Some(find.clone())
        } else {
            None
        }
    }

    pub fn add_scope(&mut self, scope: Scope) {
        self.scopes.push(scope);
    }

    pub fn no_used_variables_or_functions(&self) -> Vec<Scope> {
        self.scopes
            .iter()
            .filter(|scope| scope.uses_num == 0)
            .cloned()
            .collect::<Vec<Scope>>()
    }
}
