use llvm::builder::LLVMBuilder;
use llvm::function::function_builder::FunctionBuilder;
use llvm::function::instruction::terminator_instruction::create_function::{create_insert_function};
use llvm::linkage_types::LinkageTypes;
use llvm::llvm_type::LLVMTypes;
use llvm::llvm_util::jit;
use llvm::value::{create_number, create_ref_string, create_string};

#[test]
fn builder_global_var_string_test() {
    let mut builder = LLVMBuilder::new("test.zx");

    builder.create_global_var(
        LinkageTypes::Private,
        "abc".to_string(),
        create_string("你好"),
        false,
    ).unwrap();

    let llvm_ir = builder.to_string();
    println!("{}", llvm_ir);
}

#[test]
fn builder_global_var_int_test() {
    let mut builder = LLVMBuilder::new("test.zx");

    builder.create_global_var(
        LinkageTypes::Private,
        "a".to_string(),
        create_number("123", LLVMTypes::Int32),
        false,
    ).unwrap();

    builder.create_global_var(
        LinkageTypes::Private,
        "abc".to_string(),
        create_number("", LLVMTypes::Int64),
        false,
    ).unwrap();

    builder.add_named_mata("llvm.ident".to_string(), vec![
        create_number("0".to_string(), LLVMTypes::Int8)
    ]);

    builder.add_named_mata("0".to_string(), vec![
        create_ref_string("zx version 1".to_string())
    ]);

    let llvm_ir = builder.to_string();

    let expect_test = r##"; ModuleID = 'test.zx'
source_filename = "test.zx"

@a = private dso_local global i32 123, align 4
@abc = private dso_local global i64 , align 8


!llvm.ident = !{!0}
!0 = !{!"zx version 1"}
"##;
    assert_eq!(expect_test, &llvm_ir);
}

#[test]
fn builder_function_test() {
    let mut builder = LLVMBuilder::new("test");
    let mut function = FunctionBuilder::new("main", &[], LLVMTypes::Void);
    let str_var = builder.create_global_var(LinkageTypes::Private, "str", create_string("Hello, world!"), true).unwrap();
    let str_var = function.create_getelementptr(str_var);
    let fun_args = [LLVMTypes::get_pointer(LLVMTypes::Int8)];
    let parameters = [str_var];
    let printf_fn = create_insert_function("printf", LLVMTypes::Int32, &fun_args, true);

    builder.add_insert_function(&printf_fn);
    function.create_call(&printf_fn, &parameters);
    builder.add_function(function);


    let llvm_ir = builder.to_string();

    if let Ok(run_string) = jit(llvm_ir) {
        assert_eq!("Hello, world!", run_string);
    };
}