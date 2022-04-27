use std::fmt;
use std::fmt::Formatter;
use crate::llvm_type::LLVMTypes;

pub struct FunctionContent {}

pub struct AllocaContent {
    pub result: String,
    pub alloca_type: LLVMTypes,
    pub align: i8,
}

impl fmt::Display for AllocaContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "%{} = {}, align {}", self.result, self.alloca_type.to_string(), self.align)
    }
}