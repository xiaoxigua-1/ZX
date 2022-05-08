use crate::llvm_type::LLVMTypes;

#[derive(Clone)]
pub struct LLVMVariable {
    pub variable_name: String,
    pub result_type: LLVMTypes,
    pub is_global: bool,
}

#[derive(Clone)]
pub struct FunctionInfo<'a> {
    pub name: String,
    pub ret_type: LLVMTypes,
    pub args_types: &'a [LLVMTypes],
    pub varargs: bool,
}
