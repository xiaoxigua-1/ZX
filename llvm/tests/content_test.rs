use llvm::context::{GlobalVariableContext, LLVMContext};
use llvm::linkage_types::LinkageTypes;
use llvm::llvm_type::LLVMTypes;
use llvm::value::{Value};

#[test]
fn global_variable_context() {
    let str = GlobalVariableContext {
        linkage: LinkageTypes::NULL,
        is_constant: false,
        variable_name: "a".to_string(),
        value: Value {
            context: "2".to_string(),
            value_type: LLVMTypes::Int8,
        },
    }
        .to_string();

    let expect_test = r##"@a = dso_local global i8 2, align 1"##;

    assert_eq!(expect_test, &str);
}

#[test]
fn context_test() {
    let global_variables = vec![GlobalVariableContext {
        linkage: LinkageTypes::NULL,
        is_constant: false,
        variable_name: "a".to_string(),
        value: Value {
            context: "12".to_string(),
            value_type: LLVMTypes::Int8,
        },
    }];

    let source_filename = "test.zx".to_string();

    let llvm_ir = LLVMContext {
        source_filename,
        global_variables,
        named_metadata: vec![],
        functions: vec![],
        declarations: vec![]
    }
        .to_string();
    let expect_test = r##"; ModuleID = 'test.zx'
source_filename = "test.zx"

@a = dso_local global i8 12, align 1



"##;
    assert_eq!(expect_test, &llvm_ir);
}