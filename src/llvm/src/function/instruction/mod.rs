use crate::llvm_type::LLVMTypes;
use crate::value::Value;

pub enum Instructions {
    Ret {
        ret_type: LLVMTypes,
        value: Value
    },
    Br,
}

impl Instructions {

}