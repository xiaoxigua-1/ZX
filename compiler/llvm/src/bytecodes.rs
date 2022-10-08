use std::sync::Mutex;

use inkwell::values::FunctionValue;
use util::bytecode::BytecodeType;

use crate::Builder;

impl Builder<'_> {
    pub fn bytecodes(
        &self,
        bytecode: &BytecodeType,
        function: FunctionValue,
        index: &Mutex<usize>,
    ) {
        match &bytecode {
            BytecodeType::Block { name, bytecodes } => {
                self.block_statement(name, bytecodes, function, index)
            }
            BytecodeType::Box { bytecodes } => bytecodes
                .iter()
                .for_each(|bytecode| self.bytecodes(bytecode, function, index)),
            _ => {}
        }
    }

    pub fn block_statement(
        &self,
        name: &String,
        bytecodes: &Vec<BytecodeType>,
        function: FunctionValue,
        index: &Mutex<usize>,
    ) {
        let block = self.context.append_basic_block(function, name);
        self.builder.position_at_end(block);
        bytecodes.iter().for_each(|bytecode| {
            self.bytecodes(bytecode, function, index);
        });
    }
}
