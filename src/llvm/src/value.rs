use crate::llvm_type::LLVMTypes;
use std::fmt;

pub struct Value {
    pub context: String,
    pub value_type: LLVMTypes,
}

/// create string
/// Example: `c"abc\00"`
pub fn create_string<T: fmt::Display>(value: T) -> Value {
    Value {
        context: format!(r#"c"{}\00""#, value.to_string()),
        value_type: LLVMTypes::String {
            len: value.to_string().len(),
        },
    }
}

/// create reference string
/// Example: `"abc"`
pub fn create_ref_string<T: fmt::Display>(value: T) -> Value {
    Value {
        context: format!(r#""{}""#, value.to_string()),
        value_type: LLVMTypes::String {
            len: value.to_string().len(),
        },
    }
}

/// create number
/// Example: `123`
pub fn create_number<T: fmt::Display>(value: T, number_type: LLVMTypes) -> Value {
    Value {
        context: value.to_string(),
        value_type: number_type,
    }
}

/// create local variable
/// Example: `%1`
pub fn create_local_variable<T: fmt::Display>(name: T, local_variable_type: LLVMTypes) -> Value {
    Value {
        context: format!("%{}", name),
        value_type: local_variable_type,
    }
}

/// create global variable
/// Example: `@abc`
pub fn create_global_variable<T: fmt::Display>(name: T, global_variable_type: LLVMTypes) -> Value {
    Value {
        context: format!("@{}", name),
        value_type: global_variable_type,
    }
}

/// void
pub fn create_void() -> Value {
    Value {
        context: String::new(),
        value_type: LLVMTypes::Void,
    }
}
