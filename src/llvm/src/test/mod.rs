#[cfg(test)]
mod context_test {
    use crate::builder::LLVMBuilder;
    use crate::context::{GlobalVariableContext, LLVMContext};
    use crate::llvm_type::LLVMTypes;
    use crate::value::Value;

    #[test]
    fn global_variable_context() {
        let str = GlobalVariableContext {
            is_constant: false,
            variable_name: "a".to_string(),
            value: Value {
                context: "2".to_string(),
                is_string: false,
            },
            value_type: LLVMTypes::Int8,
        }
        .to_string();

        println!("{}", str);
    }

    #[test]
    fn context_test() {
        let global_variables = vec![GlobalVariableContext {
            is_constant: false,
            variable_name: "a".to_string(),
            value: Value {
                context: "12".to_string(),
                is_string: false,
            },
            value_type: LLVMTypes::Int8,
        }];

        let source_filename = "test.zx".to_string();

        let llvm_ir = LLVMContext {
            source_filename,
            global_variables,
        }
        .to_string();

        println!("{}", llvm_ir);
    }

    fn builder_global_var() {}

    #[test]
    fn builder_global_var_string_test() {
        let mut builder = LLVMBuilder::new("test.zx");
        let value = String::from("你好");

        builder.crate_global_var(
            "abc".to_string(),
            LLVMTypes::String { len: value.len() },
            value,
            false,
        );

        let llvm_ir = builder.to_string();
        println!("{}", llvm_ir);
    }

    #[test]
    fn builder_global_var_int_test() {
        let mut builder = LLVMBuilder::new("test.zx");
        let value = String::from("123");

        builder.crate_global_var("abc".to_string(), LLVMTypes::Int8, value, false);

        let llvm_ir = builder.to_string();
        println!("{}", llvm_ir);
    }
}
