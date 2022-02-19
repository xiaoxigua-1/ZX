mod test;

use lexer::Lexer;
use parser::Parser;

pub struct Compiler {
    path: String
}

impl Compiler {
    fn compile(&self) {
        let mut lexer = Lexer::new(&self.path);
        lexer.lexer();
        let parser = Parser { tokens: lexer.tokens };
        parser.parse();
    }
}