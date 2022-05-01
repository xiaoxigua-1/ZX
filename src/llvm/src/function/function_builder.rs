use crate::function::instruction::terminator_instruction::memory_access::MemoryAccess;
use crate::function::instruction::terminator_instruction::TerminatorInstructions;
use crate::function::info::{FunctionInfo, LLVMLocation};
use std::fmt;
use std::fmt::Formatter;
use crate::function::instruction::terminator_instruction::other::OtherInstruction;
use crate::function::instruction::terminator_instruction::TerminatorInstructions::{Block, Other};

use crate::llvm_type::LLVMTypes;
use crate::llvm_util::LLVMError;
use crate::value::{create_local_variable, create_void};

#[derive(Clone)]
pub struct FunctionBuilder<'a> {
    name: &'a str,
    arguments: &'a [LLVMTypes],
    pub index: usize,
    alloca_list: Vec<MemoryAccess>,
    instructions: Vec<TerminatorInstructions<'a>>,
    ret_type: LLVMTypes,
}

impl <'b> FunctionBuilder<'b> {
    pub fn new<'a>(
        name: &'a str,
        arguments: &'a [LLVMTypes],
        ret_type: LLVMTypes,
    ) -> FunctionBuilder<'a> {
        FunctionBuilder {
            name,
            arguments,
            index: arguments.len(),
            alloca_list: vec![],
            instructions: vec![],
            ret_type,
        }
    }

    pub fn add_alloca(&mut self, alloca_type: LLVMTypes) -> LLVMLocation {
        let id = self.alloca_list.len() + self.arguments.len() + 1;
        let align = Some(alloca_type.get_align());

        self.alloca_list.push(MemoryAccess::Alloca {
            result: id.to_string(),
            alloca_type: alloca_type.clone(),
            num: None,
            align,
        });
        LLVMLocation {
            location: id,
            result_type: alloca_type.clone(),
        }
    }

    pub fn create_call(&mut self, call_function_info: &'b FunctionInfo) -> LLVMLocation {
        let id = self.get_id() + 1;
        self.index += 1;
        self.instructions.push(Other {
            instruction: OtherInstruction::Call {
                result: id.to_string(),
                function_info: call_function_info.clone(),
            }
        });

        LLVMLocation {
            location: id.clone(),
            result_type: call_function_info.ret_type.clone()
        }
    }

    pub fn add_basic_block<T: fmt::Display>(&mut self, id: T) {
        self.instructions.push(Block { name: id.to_string() });
    }

    pub fn get_nth_param(&mut self, index: usize) -> Result<LLVMLocation, LLVMError<&str>> {
        if index < self.arguments.len() {
            let id = self.alloca_list.len() + self.arguments.len() + 1;
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

    pub fn get_id(&self) -> usize {
        self.index + self.arguments.len() + 1
    }

    pub fn add_instruction(&mut self, instruction: TerminatorInstructions<'b>) {
        self.instructions.push(instruction);
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
            "
define dso_local {} @{}({}) {{
{}
{}
}}
",
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
