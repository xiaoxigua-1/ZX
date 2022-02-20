mod test;

use lexer::Lexer;
use parser::Parser;

pub struct Compiler {
    path: String
}

impl Compiler {
    pub fn compile(&self) -> Result<(), ()> {
        let mut lexer = Lexer::new(&self.path);
        match lexer.lexer() {
            Ok(()) => {
                let mut parser = Parser::new(&lexer.tokens);
                parser.parse();
                Ok(())
            }
            Err(()) => {
                Err(())
            }
        }
    }
}
