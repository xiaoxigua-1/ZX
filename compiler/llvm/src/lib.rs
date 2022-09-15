mod statement;
mod expression;

use util::scope::Scope;
use inkwell::context::Context;
use inkwell::module::Module;
use util::ast::Statement;
use util::report::Report;

pub struct Builder<'a> {
    ast: Vec<Statement>,
    reports: Vec<Report>,
    context: &'a Context,
    module: Module<'a>,
    builder: inkwell::builder::Builder<'a>
}

impl Builder<'_> {
    pub fn new(ast: Vec<Scopes>, context: &Context) -> Builder {
        Builder {
            ast,
            reports: vec![],
            builder: context.create_builder(),
            module: context.create_module("main"),
            context,
        }
    }

    pub fn build(&self) {
        for statement in &self.ast {
            self.statement(statement)
        }
    }
}