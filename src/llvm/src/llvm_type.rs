use std::fmt;
use LLVMTypes::*;

pub enum LLVMTypes {
    Int8,
    Int16,
    Int32,
    Int64,
    Float,
    Double
}

impl LLVMTypes {
    pub fn get_align(&self) -> i8 {
        match self {
            Int8 => 1,
            Int16 => 2,
            Int32 | Float => 4,
            Int64 | Double => 8,
        }
    }
}

impl fmt::Display for LLVMTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Int8 => "i8",
            Int16 => "i16",
            Int32 => "i32",
            Int64 => "i64",
            Float => "float",
            Double => "double",
        })
    }
}