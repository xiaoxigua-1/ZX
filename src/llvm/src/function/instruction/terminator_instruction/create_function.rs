use std::fmt;
use crate::function::function_builder::FunctionBuilder;
use crate::function::instruction::terminator_instruction::TerminatorInstructions;

pub fn unconditional_br<T: fmt::Display>(id: T) -> TerminatorInstructions {
    TerminatorInstructions::UnconditionalBr {
        dest: id.to_string()
    }
}

pub fn create_basic_block(function_builder: &mut FunctionBuilder) -> usize {
    let index = function_builder.index.clone();
    function_builder.index += 1;
    index
}