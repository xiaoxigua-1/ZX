use crate::llvm_type::LLVMTypes;

pub struct LLVMLocation {
    pub location: usize,
    pub result_type: LLVMTypes,
}

#[derive(Clone)]
pub struct FunctionInfo<'a> {
    pub name: String,
    pub ret_type: LLVMTypes,
    pub args_types: &'a [LLVMTypes],
    pub varargs: bool,
}
