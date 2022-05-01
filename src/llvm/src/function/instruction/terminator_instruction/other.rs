use std::fmt;
use std::fmt::Formatter;
use OtherInstruction::*;
use crate::function::info::FunctionInfo;

#[derive(Clone)]
pub enum OtherInstruction<'a> {
    Call {
        result: String,
        function_info: FunctionInfo<'a>
    }
}

impl fmt::Display for OtherInstruction<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Call { result, function_info } => call_content(result, function_info)
            }
        )
    }
}

fn call_content(result: &String, function_info: &FunctionInfo) -> String {
    format!(
        "  %{} = call {} ({}{}) @{}()",
        result,
        function_info.ret_type.to_string(),
        function_info.args_types
            .iter()
            .map(|arg| { format!("{}*", arg.to_string()) })
            .collect::<Vec<String>>()
            .join(", "),
        if function_info.varargs {
            String::from(", ...")
        } else {
            String::new()
        },
        function_info.name
    )
}