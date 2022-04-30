use llvm::function::function_builder::FunctionBuilder;
use llvm::function::instruction::terminator_instruction::create_function::{create_basic_block, unconditional_br};
use llvm::llvm_type::LLVMTypes;
use llvm::value::{create_number};

#[test]
fn function_builder_test() {
    let mut function_builder = FunctionBuilder::new("main", &[LLVMTypes::Int32], LLVMTypes::Void);
    let value = create_number(123, LLVMTypes::Int32);
    function_builder.get_nth_param(0).unwrap();

    function_builder.create_local_variable(value);
    let basic_block = create_basic_block(&mut function_builder);
    function_builder.add_instruction(unconditional_br(basic_block));
    function_builder.add_basic_block(basic_block);
    let value = create_number(123, LLVMTypes::Int64);
    function_builder.create_local_variable(value);
    let llvm_ir = function_builder.build();
    println!("{}", llvm_ir);
}