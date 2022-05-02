use crate::linkage_types::LinkageTypes;
use crate::value::Value;
use std::fmt;
use std::fmt::Formatter;
use crate::function::function_builder::FunctionBuilder;
use crate::llvm_type::LLVMTypes;

pub struct LLVMContext<'a> {
    pub source_filename: String,
    pub global_variables: Vec<GlobalVariableContext>,
    pub named_metadata: Vec<NamedMetadata>,
    pub functions: Vec<FunctionBuilder<'a>>,
    pub declarations: Vec<Declaration<'a>>,
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

#[derive(Clone)]
pub struct Declaration<'a> {
    pub name: String,
    pub ret_type: LLVMTypes,
    pub args_types: &'a[LLVMTypes],
    pub varargs: bool,
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

impl fmt::Display for Declaration<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "declare dso_local {} @{}({}{})",
            self.ret_type.to_string(),
            self.name,
            self.args_types
                .iter()
                .map(|arg| { format!("{}", arg.to_string()) })
                .collect::<Vec<String>>()
                .join(", "),
            if self.varargs {
                String::from(", ...")
            } else {
                String::new()
            }
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

impl LLVMContext <'_> {
    pub fn to_string(&mut self) -> String {
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
        let functions_string = self.functions
            .iter_mut()
            .map(|function| { function.build() })
            .collect::<Vec<String>>()
            .join("\n");
        let declarations_string = self.declarations
            .iter()
            .map(|declaration| { declaration.to_string() })
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "\
; ModuleID = '{}'
source_filename = \"{}\"

{}
{}
{}
{}
",
            self.source_filename,
            self.source_filename,
            global_variable_string,
            functions_string,
            declarations_string,
            named_metadata_string
        )
    }
}
