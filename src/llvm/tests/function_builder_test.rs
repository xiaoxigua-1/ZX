use llvm::function::function_builder::FunctionBuilder;
use llvm::llvm_type::LLVMTypes;
use llvm::value::{create_string};

#[test]
fn function_builder_test() {
    let mut function_builder = FunctionBuilder::new("abc", &[]);
    let value = create_string(String::new());
    let id = function_builder.create_local_variable(value, LLVMTypes::Int32);
    println!("{}", id);
    function_builder.set_ret_type(LLVMTypes::Int32);
    let llvm_ir = function_builder.to_string();
    println!("{}", llvm_ir);
}