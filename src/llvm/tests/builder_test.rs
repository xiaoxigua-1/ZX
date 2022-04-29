use llvm::builder::LLVMBuilder;
use llvm::linkage_types::LinkageTypes;
use llvm::llvm_type::LLVMTypes;
use llvm::value::{create_number, create_ref_string};

#[test]
fn builder_global_var_string_test() {
    let mut builder = LLVMBuilder::new("test.zx");
    let value = String::from("你好");

    builder.crate_global_var(
        LinkageTypes::Private,
        "abc".to_string(),
        LLVMTypes::String { len: value.len() },
        value,
        false,
    ).unwrap_or_else(|error| {
        error.print_error_message()
    });

    let llvm_ir = builder.to_string();
    println!("{}", llvm_ir);
}

#[test]
fn builder_global_var_int_test() {
    let mut builder = LLVMBuilder::new("test.zx");
    let value = String::from("123");

    builder.crate_global_var(
        LinkageTypes::Private,
        "abc".to_string(),
        LLVMTypes::Int32,
        value,
        false,
    ).unwrap_or_else(|error| {
        error.print_error_message()
    });
    let value = String::new();

    builder.crate_global_var(
        LinkageTypes::Private,
        "abc".to_string(),
        LLVMTypes::Int64,
        value,
        false,
    ).unwrap();

    builder.add_named_mata("llvm.ident".to_string(), vec![
        create_number("0".to_string())
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