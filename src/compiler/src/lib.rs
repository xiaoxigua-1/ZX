mod test;

use lexer::Lexer;
use parser::Parser;

pub struct Compiler {
    path: String
}

impl Compiler {
    fn compile(&self) -> Result<(), ()> {
        let mut lexer = Lexer::new(&self.path);
        match lexer.lexer() {
            Ok(()) => {
                let parser = Parser { tokens: lexer.tokens };
                parser.parse();
                Ok(())
            }
            Err(()) => {
                Err(())
            }
        }
    }
}