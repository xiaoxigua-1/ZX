use std::fmt;
use std::fmt::Formatter;
use LLVMTypes::*;

#[derive(Clone)]
pub enum LLVMTypes {
    /// integer 8-bits type
    Int8,
    /// integer 16-bits type
    Int16,
    /// integer 32-bits type
    Int32,
    /// integer 64-bits type
    Int64,
    Float,
    Double,
    String {
        len: usize,
    },
    Void,
    Array {
        arr_type: Box<LLVMTypes>,
        len: usize,
    },
}

#[derive(Clone)]
pub struct PointerType {
    llvm_type: LLVMTypes,
}

impl LLVMTypes {
    pub fn get_align(&self) -> i8 {
        match self {
            Int8 | String { .. } => 1,
            Int16 => 2,
            Int32 | Float => 4,
            Int64 | Double => 8,
            Void => 0,
            Array { arr_type, .. } => arr_type.get_align(),
        }
    }
}

impl fmt::Display for LLVMTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Int8 => "i8".to_string(),
                Int16 => "i16".to_string(),
                Int32 => "i32".to_string(),
                Int64 => "i64".to_string(),
                Float => "float".to_string(),
                Double => "double".to_string(),
                Void => "void".to_string(),
                String { len } => format!("[{} x i8]", len + 1),
                Array { arr_type, len } => format!("[{} x {}]", len, arr_type.to_string()),
            }
        )
    }
}

impl fmt::Display for PointerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}*",
            self.llvm_type.to_string()
        )
    }
}
