use std::fmt;
use std::fmt::Formatter;

pub struct LLVMError<T> {
    pub message: T,
}

impl <T: fmt::Display> fmt::Debug for LLVMError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[31m{}\x1b[0m", self.message)
    }
}

impl <T: fmt::Display> LLVMError<T> {
    pub fn print_error_message(&self) {
        println!("\x1b[31merror: {}\x1b[0m", self.message)
    }
}
