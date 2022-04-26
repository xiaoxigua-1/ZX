use crate::llvm_type::LLVMTypes;

pub struct FunctionContent {}

pub struct AllocaContent {
    alloca_type: LLVMTypes,
    align: i32,
}