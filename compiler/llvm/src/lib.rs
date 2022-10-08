mod bytecodes;
mod scope;
mod struct_type;

use std::path::Path;

pub use inkwell::context::Context;
use inkwell::module::Module;
use struct_type::Structs;
use util::report::Report;
use util::scope::Scopes;

pub struct Builder<'a> {
    scopes: Scopes,
    structs: Structs<'a>,
    reports: Vec<Report>,
    context: &'a Context,
    module: Module<'a>,
    builder: inkwell::builder::Builder<'a>,
}

impl Builder<'_> {
    pub fn new(scopes: Scopes, context: &Context) -> Builder {
        Builder {
            scopes,
            reports: vec![],
            structs: Structs::new(),
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

    pub fn compile(&self, path: &Path) {
        self.module.write_bitcode_to_path(path);
    }
}
