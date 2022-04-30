use llvm::function::function_builder::FunctionBuilder;
use llvm::llvm_type::LLVMTypes;
use llvm::value::{create_number};

#[test]
fn function_builder_test() {
    let mut function_builder = FunctionBuilder::new("abc", &[LLVMTypes::Int32], LLVMTypes::Void);
    let value = create_number("123");
    function_builder.create_local_variable(value, LLVMTypes::Int32);
    function_builder.get_nth_param(0).unwrap();
    let llvm_ir = function_builder.build();
    println!("{}", llvm_ir);
}