use crate::llvm_type::LLVMTypes;
use crate::llvm_util::align_content;
use crate::value::Value;
use std::fmt;
use std::fmt::Formatter;
use MemoryAccess::*;
use crate::function::info::LLVMVariable;

/// A key design point of an SSA-based representation is how it represents memory.
/// In LLVM, no memory locations are in SSA form, which makes things very simple.
/// This section describes how to read, write, and allocate memory in LLVM.
#[derive(Clone)]
pub enum MemoryAccess {
    /// The ‘alloca’ instruction allocates memory on the stack frame of the currently executing function, to be automatically released when this function returns to its caller.
    /// If the address space is not explicitly specified, the object is allocated in the alloca address space from the datalayout string.
    Alloca {
        result: String,
        alloca_type: LLVMTypes,
        num: Option<i8>,
        align: Option<i8>,
    },
    /// The ‘load’ instruction is used to read from memory.
    Load {
        result: String,
        load_type: LLVMTypes,
        pointer: String,
        align: Option<i8>,
    },
    /// The ‘store’ instruction is used to write to memory.
    Store {
        value: Value,
        pointer: String,
        align: Option<i8>,
    },
    Getelementptr {
        result: String,
        variable: LLVMVariable
    }
}

impl fmt::Display for MemoryAccess {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Alloca {
                    result,
                    alloca_type,
                    num,
                    align,
                } => alloca_content(result, alloca_type, num, align),
                Load {
                    result,
                    load_type,
                    pointer,
                    align,
                } => load_content(result, load_type, pointer, align),
                Store {
                    value,
                    pointer,
                    align,
                } => store_content(value, &value.value_type.clone(), pointer, align),
                Getelementptr { result, variable } => getelementptr_content(result, variable)
            }
        )
    }
}

fn alloca_content(
    result: &String,
    alloca_type: &LLVMTypes,
    num: &Option<i8>,
    align: &Option<i8>,
) -> String {
    let type_string = alloca_type.to_string();
    let num_elements_string = if let Some(num) = num {
        format!(", {} {}", type_string, num)
    } else {
        String::new()
    };
    let align_string = align_content(align);

    format!(
        "  %{} = alloca {}{}{}",
        result, type_string, num_elements_string, align_string
    )
}

fn load_content(
    result: &String,
    load_type: &LLVMTypes,
    pointer: &String,
    align: &Option<i8>,
) -> String {
    let load_type_string = load_type.to_string();
    let align_string = align_content(align);

    format!(
        "  ${} = load {}, {}* {}{}",
        result, load_type_string, load_type_string, pointer, align_string
    )
}

fn store_content(
    value: &Value,
    value_type: &LLVMTypes,
    pointer: &String,
    align: &Option<i8>,
) -> String {
    let load_type_string = value_type.to_string();
    let align_string = align_content(align);

    format!(
        "  store {} {}, {}* %{}{}",
        load_type_string, value.context, load_type_string, pointer, align_string
    )
}

fn getelementptr_content(result: &String, value: &LLVMVariable) -> String {
    match &value.result_type {
        LLVMTypes::String { .. } => format!(
            "  %{} = getelementptr {}, {}* @{}, i64 0, i64 0",
            result,
            &value.result_type.to_string(),
            &value.result_type.to_string(),
            value.variable_name
        ),
        _ => String::new()
    }
}