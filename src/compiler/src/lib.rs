mod test;

use std::fs;
use lexer::Lexer;
use parser::Parser;
use check::Checker;
use util::repost::{Report, Level};

pub struct Compiler {
    path: String
}

impl Compiler {
    pub fn compile(&self) -> Result<(), ()> {
        let source = fs::read_to_string(&self.path).expect("Something went wrong reading the file");
        let mut lexer = Lexer::new(&source);

        let mut check = match lexer.lexer() {
            Ok(()) => {
                let mut parser = Parser::new(&lexer.tokens);
                parser.parse(&self.path, &source);
                Checker::new(parser.asts)
            }
            Err(error) => {
                // report lexer error
                Report { level: Level::Error, error }.print(&source, &self.path);
                return Err(())
            }
        };

        check.check();

        for repost in check.reposts.iter() {
            repost.print(&source, &self.path)
        }

        Ok(())
    }
}
