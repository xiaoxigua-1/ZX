use std::fmt;
use LLVMTypes::*;

pub enum LLVMTypes {
    Int8,
    Int16,
    Int32,
    Int64,
    Float,
    Double,
    String {
        len: usize
    },
}

impl LLVMTypes {
    pub fn get_align(&self) -> i8 {
        match self {
            Int8 | String { .. } => 1,
            Int16 => 2,
            Int32 | Float => 4,
            Int64 | Double => 8,
        }
    }
}

impl fmt::Display for LLVMTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Int8 => "i8".to_string(),
            Int16 => "i16".to_string(),
            Int32 => "i32".to_string(),
            Int64 => "i64".to_string(),
            Float => "float".to_string(),
            Double => "double".to_string(),
            String { len } => format!("[{} x i8]", len + 1)
        })
    }
}