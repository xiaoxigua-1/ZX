use crate::zx_type::ZXTyped;

#[derive(Clone, Debug)]
pub enum BytecodeType {
    Call {
        path: String,
        ret_type: ZXTyped,
        argument_types: Vec<ZXTyped>,
    },
    Alloca {
        path: String,
        alloca_type: ZXTyped,
    },
    Store {
        path: String,
        value: BytecodeValue,
    },
    Load {
        path: String,
    },
    Ret {
        path: String,
    },
    Block {
        name: String,
        bytecodes: Vec<BytecodeType>,
    },
    Box {
        bytecodes: Vec<BytecodeType>,
    },
    Value {
        value: BytecodeValue,
    },
}

impl BytecodeType {
    pub fn int_value(value: i32) -> BytecodeType {
        BytecodeType::Value {
            value: BytecodeValue::Int { value },
        }
    }

    pub fn char_value(value: char) -> BytecodeType {
        BytecodeType::Value {
            value: BytecodeValue::Char { value },
        }
    }

    pub fn float_value(value: f32) -> BytecodeType {
        BytecodeType::Value {
            value: BytecodeValue::Float { value },
        }
    }

    pub fn bool_value(value: bool) -> BytecodeType {
        BytecodeType::Value {
            value: BytecodeValue::Bool { value },
        }
    }

    pub fn string_value(value: String) -> BytecodeType {
        BytecodeType::Value {
            value: BytecodeValue::String { value },
        }
    }

    pub fn param_value(index: i32) -> BytecodeType {
        BytecodeType::Value {
            value: BytecodeValue::Param { index },
        }
    }
}

#[derive(Clone, Debug)]
pub enum BytecodeValue {
    Int { value: i32 },
    Float { value: f32 },
    Bool { value: bool },
    String { value: String },
    Char { value: char },
    PointerValue { path: String },
    Param { index: i32 },
}
