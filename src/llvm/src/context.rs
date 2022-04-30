use crate::linkage_types::LinkageTypes;
use crate::value::Value;
use std::fmt;
use std::fmt::Formatter;

pub struct LLVMContext {
    pub source_filename: String,
    pub global_variables: Vec<GlobalVariableContext>,
    pub named_metadata: Vec<NamedMetadata>,
}

pub struct NamedMetadata {
    pub name: String,
    pub value: Vec<Value>,
}

pub struct GlobalVariableContext {
    pub linkage: LinkageTypes,
    pub is_constant: bool,
    pub variable_name: String,
    pub value: Value,
}

impl fmt::Display for GlobalVariableContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "@{} ={} dso_local {} {} {}, align {}",
            self.variable_name,
            self.linkage.to_string(),
            if self.is_constant {
                "constant"
            } else {
                "global"
            },
            self.value.value_type,
            self.value.context,
            self.value.value_type.get_align()
        )
    }
}

impl fmt::Display for NamedMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let named_metadata_string = self
            .value
            .iter()
            .map(|value| {
                let value_string = &value.context;
                format!("!{}", value_string)
            })
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "!{} = !{{{}}}", self.name, named_metadata_string)
    }
}

impl fmt::Display for LLVMContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let global_variable_string = self
            .global_variables
            .iter()
            .map(|global_variable| global_variable.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let named_metadata_string = self
            .named_metadata
            .iter()
            .map(|named_metadata| named_metadata.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(
            f,
            "\
; ModuleID = '{}'
source_filename = \"{}\"

{}

{}
",
            self.source_filename,
            self.source_filename,
            global_variable_string,
            named_metadata_string
        )
    }
}
