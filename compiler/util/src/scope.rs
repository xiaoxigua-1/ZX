use crate::zx_type::ZXTyped;
use crate::ast::{Parameter, Statement};
use crate::token::Position;

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
        members: Scopes,
    },
    Block {
        children: Scopes,
        ret: (ZXTyped, Option<Position>)
    }
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub name: String,
    pub pos: Position,
    pub scope_type: ScopeType,
    pub uses_num: i32,
}

#[derive(Debug, Clone)]
pub struct Scopes {
    pub scopes: Vec<Scope>,
}

impl Scopes {
    pub fn new() -> Scopes {
        Scopes { scopes: vec![] }
    }

    pub fn find_scope(&mut self, name: &String) -> Option<Scope> {
        let mut find: Option<Scope> = None;
        for scope in self.scopes.iter_mut() {
            if scope.name.eq(name) {
                scope.uses_num += 1;
                find = Some(scope.clone())
            }
        }

        if let Some(last_scope) = self.scopes.last_mut() {
            if let Some(find) = match &mut last_scope.scope_type {
                ScopeType::Block { children, .. } => children.find_scope(name),
                ScopeType::DefClass { members } => members.find_scope(name),
                _ => None,
            } {
                return Some(find);
            };
        };

        find
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
