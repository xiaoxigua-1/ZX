use crate::context::{GlobalVariableContext, LLVMContext, NamedMetadata};
use crate::llvm_type::LLVMTypes;
use crate::value::{create_number, create_string, Value};
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
                named_metadata: vec![]
            },
        }
    }

    pub fn crate_global_var(
        &mut self,
        variable_name: String,
        value_type: LLVMTypes,
        value: String,
        is_constant: bool,
        is_private: bool
    ) {
        self.context.global_variables.push(GlobalVariableContext {
            is_private,
            is_constant,
            variable_name,
            value: if let LLVMTypes::String { .. } = value_type {
                create_string(value)
            } else {
                create_number(value)
            },
            value_type,
        });
    }

    pub fn add_named_mata(&mut self, name: String, value: Vec<Value>) {
        self.context.named_metadata.push(NamedMetadata {
            name,
            value
        });
    }
}

impl fmt::Display for LLVMBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.context.to_string())
    }
}
