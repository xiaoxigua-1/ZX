pub mod memory_access;

use crate::llvm_type::LLVMTypes;
use crate::value::Value;
use std::fmt;
use std::fmt::Formatter;
use TerminatorInstructions::*;

pub enum TerminatorInstructions {
    /// The ‘ret’ instruction is used to return control flow (and optionally a value) from a function back to the caller.
    // There are two forms of the ‘ret’ instruction: one that returns a value and then causes control flow, and one that just causes control flow to occur.
    Ret { ret_type: LLVMTypes, value: Value },
    /// The ‘br’ instruction is used to cause control flow to transfer to a different basic block in the current function.
    /// corresponding to a conditional branch
    Br {
        cond: String,
        if_true: String,
        if_false: String,
    },
    /// The ‘br’ instruction is used to cause control flow to transfer to a different basic block in the current function.
    /// corresponding to a unconditional branch.
    UnconditionalBr { dest: String },
    /// A key design point of an SSA-based representation is how it represents memory.
    /// In LLVM, no memory locations are in SSA form, which makes things very simple.
    /// This section describes how to read, write, and allocate memory in LLVM.
    MemoryAccess {
        instruction: memory_access::MemoryAccess,
    },
}

impl fmt::Display for TerminatorInstructions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Ret { ret_type, value } => ret_content(ret_type, value),
                MemoryAccess { instruction } => instruction.to_string(),
                UnconditionalBr { dest } => format!("  br %{}", dest),
                Br {
                    cond,
                    if_false,
                    if_true,
                } => format!("  br i1 %{}, label %{}, label %{}", cond, if_true, if_false),
            }
        )
    }
}

fn ret_content(ret_type: &LLVMTypes, value: &Value) -> String {
    match ret_type {
        LLVMTypes::Void => format!("  ret void"),
        _ => format!("  ret {} {}", ret_type.to_string(), value.context),
    }
}
