use std::fmt;
use crate::context::{GlobalVariableContext, LLVMContext, NamedMetadata};
use crate::linkage_types::LinkageTypes;
use crate::llvm_type::LLVMTypes;
use crate::llvm_util::LLVMError;
use crate::value::{create_number, Value};
use crate::function::function_builder::FunctionBuilder;

pub struct LLVMBuilder<'a> {
    context: LLVMContext<'a>,
}

impl <'a> LLVMBuilder<'a> {
    pub fn new(module_name: &str) -> LLVMBuilder {
        LLVMBuilder {
            context: LLVMContext {
                source_filename: module_name.to_string(),
                global_variables: vec![],
                named_metadata: vec![],
                functions: vec![],
            },
        }
    }

    pub fn crate_global_var<T: fmt::Display>(
        &mut self,
        linkage: LinkageTypes,
        variable_name: T,
        value: Value,
        is_constant: bool,
    ) -> Result<(), LLVMError<String>> {
        let variable_name = variable_name.to_string();
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
                value: if let LLVMTypes::String { .. } = &value.value_type {
                    value
                } else {
                    create_number(value.context, value.value_type)
                },
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

    pub fn add_function(&mut self, function: FunctionBuilder<'a>) {
        self.context.functions.push(function)
    }
}

impl LLVMBuilder <'_> {
    pub fn to_string(&mut self) -> String {
        format!("{}", self.context.to_string())
    }
}
