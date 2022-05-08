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
    Pointer {
        llvm_type: Box<LLVMTypes>,
    },
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
            Pointer { llvm_type } => llvm_type.get_align(),
        }
    }

    pub fn get_pointer(llvm_type: LLVMTypes) -> LLVMTypes {
        LLVMTypes::Pointer {
            llvm_type: Box::new(llvm_type),
        }
    }
}

impl fmt::Display for LLVMTypes {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
                Pointer { llvm_type } => format!("{}*", llvm_type),
            }
        )
    }
}
