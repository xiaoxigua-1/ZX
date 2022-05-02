use std::fmt;
use crate::function::function_builder::FunctionBuilder;
use crate::function::info::{FunctionInfo, LLVMVariable};
use crate::function::instruction::terminator_instruction::{memory_access, TerminatorInstructions};
use crate::llvm_type::LLVMTypes;
use crate::value::Value;

pub fn unconditional_br<T: fmt::Display>(id: T) -> TerminatorInstructions<'static> {
    TerminatorInstructions::UnconditionalBr {
        dest: id.to_string()
    }
}

pub fn create_basic_block(function_builder: &mut FunctionBuilder) -> usize {
    function_builder.get_id()
}

pub fn create_insert_function<T: fmt::Display>(name: T, ret_type: LLVMTypes, args_types: &[LLVMTypes], varargs: bool) -> FunctionInfo {
    FunctionInfo {
        name: name.to_string(),
        ret_type,
        args_types,
        varargs
    }
}

pub fn create_store_value(location: LLVMVariable, value: Value) -> TerminatorInstructions<'static> {
    TerminatorInstructions::MemoryAccess {
        instruction: memory_access::MemoryAccess::Store {
            value,
            pointer: location.variable_name.to_string(),
            align: Some(location.result_type.get_align())
        }
    }
}