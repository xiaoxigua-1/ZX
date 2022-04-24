use std::fmt;
use std::fmt::Formatter;

pub struct LLVMError {
    pub message: String
}

impl fmt::Debug for LLVMError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[31m{}\x1b[0m", self.message)
    }
}