use crate::function::info::{FunctionInfo, LLVMVariable};
use std::fmt;
use std::fmt::Formatter;
use OtherInstruction::*;

#[derive(Clone)]
pub enum OtherInstruction<'a> {
    Call {
        result: String,
        function_info: FunctionInfo<'a>,
        parameters: &'a [LLVMVariable],
    },
}

impl fmt::Display for OtherInstruction<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Call {
                    result,
                    function_info,
                    parameters,
                } => call_content(result, function_info, parameters),
            }
        )
    }
}

fn call_content(
    result: &String,
    function_info: &FunctionInfo,
    parameters: &[LLVMVariable],
) -> String {
    format!(
        "  %{} = call {} ({}{}) @{}({})",
        result,
        function_info.ret_type.to_string(),
        function_info
            .args_types
            .iter()
            .map(|arg| { format!("{}", arg.to_string()) })
            .collect::<Vec<String>>()
            .join(", "),
        if function_info.varargs {
            String::from(", ...")
        } else {
            String::new()
        },
        function_info.name,
        parameters
            .iter()
            .map(|paras| {
                format!(
                    "{} {}",
                    paras.result_type.to_string(),
                    format!(
                        "{}{}",
                        if paras.is_global { "@" } else { "%" },
                        paras.variable_name.to_string()
                    )
                )
            })
            .collect::<Vec<String>>()
            .join(", ")
    )
}
