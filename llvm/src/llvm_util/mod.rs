use std::{fmt};
use std::fmt::Formatter;
use std::io::{Write};
use std::process::{Command, Stdio};

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

pub fn jit(llvm_ir: String) -> Result<String, std::io::Error> {
    let mut put_command = Command::new("lli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut stdin = put_command.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin.write_all(llvm_ir.as_bytes()).expect("Failed to write to stdin");
    });
    let output = put_command.wait_with_output().expect("Failed to read stdout");
    let str = String::from_utf8(output.stdout).unwrap();
    println!("{}", str);
    Ok(str)
}
