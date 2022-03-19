mod test;

use std::fs;
use lexer::Lexer;
use parser::Parser;
use check::Checker;
use util::repost::{Repost, Level};

pub struct Compiler {
    path: String
}

impl Compiler {
    pub fn compile(&self) -> Result<(), ()> {
        let source = fs::read_to_string(&self.path).expect("Something went wrong reading the file");
        let mut lexer = Lexer::new(&source);

        match lexer.lexer() {
            Ok(()) => {
                let mut parser = Parser::new(&lexer.tokens);
                parser.parse(&self.path, &source);
                Checker::new(parser.asts);
                Ok(())
            }
            Err(error) => {
                Repost { level: Level::Error, error }.print(&source, &self.path);
                Err(())
            }
        }
    }
}
