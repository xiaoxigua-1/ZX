use std::fmt;
use std::fmt::Formatter;
use crate::llvm_type::LLVMTypes;
use crate::value::Value;
use Instructions::*;

pub enum Instructions {
    Ret {
        ret_type: LLVMTypes,
        value: Value,
    },
    Br,
    Alloca {
        result: String,
        alloca_type: LLVMTypes,
        align: i8,
    },
}

impl fmt::Display for Instructions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Alloca { result, alloca_type, align } => alloca_content(result, alloca_type, align),
                _ => String::new()
            }
        )
    }
}

fn alloca_content(result: &String, alloca_type: &LLVMTypes, align: &i8) -> String {
    format!("%{} = {}, align {}", result, alloca_type.to_string(), align)
}