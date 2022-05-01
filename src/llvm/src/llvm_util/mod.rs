use std::fmt;
use std::fmt::Formatter;
use std::process::Command;

pub struct LLVMError<T> {
    pub message: T,
}

impl<T: fmt::Display> fmt::Debug for LLVMError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[31m{}\x1b[0m", self.message)
    }
}

impl<T: fmt::Display> LLVMError<T> {
    pub fn print_error_message(&self) {
        println!("\x1b[31mLLVM IR error: {}\x1b[0m", self.message)
    }
}

pub fn align_content(align: &Option<i8>) -> String {
    if let Some(align) = align {
        format!(", align {}", align)
    } else {
        String::new()
    }
}
pub fn jit(llvm_ir: String) {
    Command::new("lli")
        .args(["<".to_string(), llvm_ir])
        .output()
        .expect("failed to execute process");
}
