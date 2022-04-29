use std::fmt;
use std::fmt::Formatter;
use crate::function::instruction::Instructions;
use crate::llvm_type::LLVMTypes;
use crate::llvm_util::LLVMError;
use crate::value::Value;

pub struct FunctionBuilder<'a> {
    name: &'a str,
    arguments: &'a [LLVMTypes],
    index: usize,
    alloca_list: Vec<Instructions>,
    ret_type: LLVMTypes
}

impl FunctionBuilder<'_> {
    pub fn new<'a>(name: &'a str, arguments: &'a [LLVMTypes]) -> FunctionBuilder<'a> {
        FunctionBuilder {
            name,
            arguments,
            index: arguments.len() + 1,
            alloca_list: vec![],
            ret_type: LLVMTypes::Void
        }
    }

    pub fn create_local_variable(&mut self, value: Value, variable_type: LLVMTypes) -> usize {
        let id = self.index.clone();
        self.index += 1;

        self.alloca_list.push(Instructions::Alloca {
            result: id.to_string(),
            alloca_type: variable_type.clone(),
            align: variable_type.get_align()
        });

        id
    }

    pub fn get_nth_param(&mut self, index: usize) -> Result<usize, LLVMError<&str>> {
        if index < self.arguments.len() {
            self.alloca_list.push(Instructions::Alloca {
                result: index.to_string(),
                alloca_type: self.arguments[index].clone(),
                align: self.arguments[index].get_align()
            });

            Ok(index)
        } else {
            Err(LLVMError { message: "No such thing" })
        }
    }

    pub fn set_ret_type(&mut self, ret_type: LLVMTypes) {
        self.ret_type = ret_type;
    }
}

impl fmt::Display for FunctionBuilder<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "define dso_local {} @{}({}) {{\n\n}}",
            self.ret_type.to_string(),
            self.name,
            self.arguments
                .iter()
                .enumerate()
                .map(|(index, arg)| {
                    format!("{} %{}", arg.to_string(), index)
                })
                .collect::<Vec<String>>()
                .join("")
        )
    }
}