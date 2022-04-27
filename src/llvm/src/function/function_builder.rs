use crate::function::content::AllocaContent;
use crate::llvm_type::LLVMTypes;
use crate::value::Value;

pub struct FunctionBuilder {
    index: i32,
    alloca_list: Vec<AllocaContent>
}

impl FunctionBuilder {
    pub fn new() -> FunctionBuilder {
        FunctionBuilder {
            index: 0,
            alloca_list: vec![]
        }
    }

    pub fn create_local_variable(&mut self, value: Value, variable_type: LLVMTypes) -> i32 {
        let id = self.index.clone();
        self.index += 1;

        self.alloca_list.push(AllocaContent {
            result: id.to_string(),
            alloca_type: variable_type.clone(),
            align: variable_type.get_align()
        });

        id
    }
}