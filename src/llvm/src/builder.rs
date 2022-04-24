use crate::context::{GlobalVariableContext, LLVMContext};
use crate::llvm_type::LLVMTypes;
use crate::value::{create_int, create_string};
use std::fmt;
use std::fmt::Formatter;

pub struct LLVMBuilder {
    context: LLVMContext,
}

impl LLVMBuilder {
    pub fn new(module_name: &str) -> LLVMBuilder {
        LLVMBuilder {
            context: LLVMContext {
                source_filename: module_name.to_string(),
                global_variables: vec![],
            },
        }
    }

    pub fn crate_global_var(
        &mut self,
        variable_name: String,
        value_type: LLVMTypes,
        value: String,
        is_constant: bool,
    ) {
        self.context.global_variables.push(GlobalVariableContext {
            is_constant,
            variable_name,
            value: if let LLVMTypes::String { .. } = value_type {
                create_string(value)
            } else {
                create_int(value)
            },
            value_type,
        });
    }
}

impl fmt::Display for LLVMBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.context.to_string())
    }
}
