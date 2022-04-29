use llvm::function::function_builder::FunctionBuilder;
use llvm::llvm_type::LLVMTypes;
use llvm::value::{create_number};

#[test]
fn function_builder_test() {
    let mut function_builder = FunctionBuilder::new("abc", &[], LLVMTypes::Void);
    let value = create_number(String::from("123"));
    function_builder.create_local_variable(value, LLVMTypes::Int32);
    let llvm_ir = function_builder.to_string();
    println!("{}", llvm_ir);
}