mod scope;
mod statement;

use inkwell::context::Context;
use inkwell::module::Module;
use util::report::Report;
use util::scope::Scopes;

pub struct Builder<'a> {
    scopes: Scopes,
    reports: Vec<Report>,
    context: &'a Context,
    module: Module<'a>,
    builder: inkwell::builder::Builder<'a>
}

impl Builder<'_> {
    pub fn new(scopes: Scopes, context: &Context) -> Builder {
        Builder {
            scopes,
            reports: vec![],
            builder: context.create_builder(),
            module: context.create_module("main"),
            context,
        }
    }

    pub fn build(&self) {
        for scope in &self.scopes.scopes {
            self.scope(scope)
        }
    }
}