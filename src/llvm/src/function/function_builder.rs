use crate::function::instruction::terminator_instruction::memory_access::MemoryAccess;
use crate::function::instruction::terminator_instruction::TerminatorInstructions;
use crate::function::location::LLVMLocation;
use std::fmt;
use std::fmt::Formatter;

use crate::llvm_type::LLVMTypes;
use crate::llvm_util::LLVMError;
use crate::value::{create_local_variable, create_void, Value};

pub struct FunctionBuilder<'a> {
    name: &'a str,
    arguments: &'a [LLVMTypes],
    index: usize,
    alloca_list: Vec<MemoryAccess>,
    instructions: Vec<TerminatorInstructions>,
    ret_type: LLVMTypes,
}

impl FunctionBuilder<'_> {
    pub fn new<'a>(
        name: &'a str,
        arguments: &'a [LLVMTypes],
        ret_type: LLVMTypes,
    ) -> FunctionBuilder<'a> {
        FunctionBuilder {
            name,
            arguments,
            index: arguments.len() + 1,
            alloca_list: vec![],
            instructions: vec![],
            ret_type,
        }
    }

    pub fn create_local_variable(&mut self, value: Value) -> LLVMLocation {
        let id = self.index.clone();
        let align = Some(value.value_type.get_align());
        let value_type = value.value_type.clone();
        self.index += 1;

        self.alloca_list.push(MemoryAccess::Alloca {
            result: id.to_string(),
            alloca_type: value_type.clone(),
            num: None,
            align,
        });
        self.instructions
            .push(TerminatorInstructions::MemoryAccess {
                instruction: MemoryAccess::Store {
                    value,
                    pointer: id.to_string(),
                    align,
                },
            });

        LLVMLocation {
            location: id,
            result_type: value_type.clone(),
        }
    }

    pub fn get_nth_param(&mut self, index: usize) -> Result<LLVMLocation, LLVMError<&str>> {
        if index < self.arguments.len() {
            let id = self.index.clone();
            let argument_type = &self.arguments[index];
            let align = Some(self.arguments[index].get_align());
            self.index += 1;
            self.alloca_list.push(MemoryAccess::Alloca {
                result: id.to_string(),
                alloca_type: argument_type.clone(),
                num: None,
                align,
            });
            self.instructions
                .push(TerminatorInstructions::MemoryAccess {
                    instruction: MemoryAccess::Store {
                        value: create_local_variable(index.to_string(), argument_type.clone()),
                        pointer: id.to_string(),
                        align,
                    },
                });

            Ok(LLVMLocation {
                location: id,
                result_type: argument_type.clone(),
            })
        } else {
            Err(LLVMError {
                message: "No such thing",
            })
        }
    }

    pub fn build(&mut self) -> String {
        match &self.ret_type {
            LLVMTypes::Void => self.instructions.push(TerminatorInstructions::Ret {
                ret_type: LLVMTypes::Void,
                value: create_void(),
            }),
            _ => {}
        };
        self.to_string()
    }
}

impl fmt::Display for FunctionBuilder<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "define dso_local {} @{}({}) {{
{}
{}
}}",
            self.ret_type.to_string(),
            self.name,
            self.arguments
                .iter()
                .enumerate()
                .map(|(index, arg)| { format!("{} %{}", arg.to_string(), index) })
                .collect::<Vec<String>>()
                .join(""),
            self.alloca_list
                .iter()
                .map(|alloca| { alloca.to_string() })
                .collect::<Vec<String>>()
                .join("\n"),
            self.instructions
                .iter()
                .map(|instruction| { instruction.to_string() })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
