use crate::context::{GlobalVariableContext, LLVMContext, NamedMetadata};
use crate::linkage_types::LinkageTypes;
use crate::llvm_type::LLVMTypes;
use crate::llvm_util::LLVMError;
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
                named_metadata: vec![],
            },
        }
    }

    pub fn crate_global_var(
        &mut self,
        linkage: LinkageTypes,
        variable_name: String,
        value_type: LLVMTypes,
        value: String,
        is_constant: bool,
    ) -> Result<(), LLVMError> {
        if self
            .context
            .global_variables
            .iter()
            .find(|var| var.variable_name.eq(&variable_name))
            .is_none()
        {
            self.context.global_variables.push(GlobalVariableContext {
                linkage,
                is_constant,
                variable_name,
                value: if let LLVMTypes::String { .. } = value_type {
                    create_string(value)
                } else {
                    create_number(value)
                },
                value_type,
            });
            Ok(())
        } else {
            Err(LLVMError {
                message: format!("redefinition of global variable '{}'", variable_name),
            })
        }
    }

    pub fn add_named_mata(&mut self, name: String, value: Vec<Value>) {
        self.context
            .named_metadata
            .push(NamedMetadata { name, value });
    }
}

impl fmt::Display for LLVMBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.context.to_string())
    }
}
