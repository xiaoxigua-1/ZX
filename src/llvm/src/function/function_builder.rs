use crate::function::instruction::terminator_instruction::memory_access;
use crate::function::instruction::terminator_instruction::TerminatorInstructions;
use std::fmt;
use std::fmt::Formatter;
use crate::function::instruction::terminator_instruction::memory_access::MemoryAccess;

use crate::llvm_type::LLVMTypes;
use crate::llvm_util::LLVMError;
use crate::value::Value;

pub struct FunctionBuilder<'a> {
    name: &'a str,
    arguments: &'a [LLVMTypes],
    index: usize,
    alloca_list: Vec<memory_access::MemoryAccess>,
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

    pub fn create_local_variable(&mut self, value: Value, variable_type: LLVMTypes) -> usize {
        let id = self.index.clone();
        let align = Some(variable_type.get_align());
        self.index += 1;

        self.alloca_list.push(MemoryAccess::Alloca {
            result: id.to_string(),
            alloca_type: variable_type.clone(),
            num: None,
            align,
        });
        self.instructions.push(TerminatorInstructions::MemoryAccess {
            instruction: MemoryAccess::Store {
                value,
                value_type: variable_type,
                pointer: id.to_string(),
                align,
            }
        });

        id
    }

    pub fn get_nth_param(&mut self, index: usize) -> Result<usize, LLVMError<&str>> {
        if index < self.arguments.len() {
            self.alloca_list.push(MemoryAccess::Alloca {
                result: index.to_string(),
                alloca_type: self.arguments[index].clone(),
                num: None,
                align: Some(self.arguments[index].get_align()),
            });

            Ok(index)
        } else {
            Err(LLVMError {
                message: "No such thing",
            })
        }
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
