mod file_stream;
mod chars;
pub mod tokens;

use std::fs;
use file_stream::FileStream;

struct Lexer {
    path: String,
}


impl Lexer {
    fn new(path: String) -> Lexer {
        Lexer {
            path: path.clone()
        }
    }

    fn lexer(mut self) {
        let _ = fs::read_to_string(self.path).expect("Something went wrong reading the file");
    }
}
