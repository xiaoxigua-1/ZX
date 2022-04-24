use std::fmt;
use std::fmt::{Formatter};
use crate::llvm_type::LLVMTypes;
use crate::value::{Value, ValueType};

pub struct LLVMContext {
    pub source_filename: String,
    pub global_variables: Vec<GlobalVariableContext>,
}

pub struct GlobalVariableContext {
    pub is_constant: bool,
    pub variable_name: String,
    pub value: Value,
    pub value_type: LLVMTypes,
}

pub struct NamedMetadata {
    pub name: String,
    pub values: Vec<ValueType>
}

impl fmt::Display for GlobalVariableContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "@{} = dso_local {} {} {}, align {}\n",
            self.variable_name,
            if self.is_constant { "constant" } else { "global" },
            self.value_type.to_string(),
            self.value,
            self.value_type.get_align()
        )
    }
}

impl fmt::Display for LLVMContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let global_variable_string = self.global_variables
            .iter()
            .map(|global_variable| { global_variable.to_string() })
            .collect::<Vec<String>>()
            .join("\n");
        write!(
            f,
            "\
; ModuleID = '{}'
source_filename = \"{}\"
{}
            ",
            self.source_filename,
            self.source_filename,
            global_variable_string
        )
    }
}