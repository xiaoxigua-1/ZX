use std::fmt;
use std::fmt::{Formatter};
use crate::context::{GlobalVariableContext, LLVMContext};
use crate::llvm_type::LLVMTypes;

pub struct LLVMBuilder {
    context: LLVMContext
}

impl LLVMBuilder {
    pub fn new(module_name: &str) -> LLVMBuilder {
        LLVMBuilder {
            context: LLVMContext {
                source_filename: module_name.to_string(),
                global_variables: vec![]
            }
        }
    }

    pub fn crate_global_var(&mut self, variable_name: String, value_type: LLVMTypes, value: String, is_constant: bool) {
        self.context.global_variables.push(
            GlobalVariableContext {
                is_constant,
                variable_name,
                value,
                value_type
            }
        );
    }
}

impl fmt::Display for LLVMBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.context.to_string())
    }
}