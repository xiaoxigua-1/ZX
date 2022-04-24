#[cfg(test)]
mod context_test {
    use crate::builder::LLVMBuilder;
    use crate::context::{GlobalVariableContext, LLVMContext};
    use crate::llvm_type::LLVMTypes;
    use crate::value::{create_number, create_ref_string, Value, ValueType};

    #[test]
    fn global_variable_context() {
        let str = GlobalVariableContext {
            is_private: false,
            is_constant: false,
            variable_name: "a".to_string(),
            value: Value {
                context: "2".to_string(),
                value_type: ValueType::Other,
            },
            value_type: LLVMTypes::Int8,
        }
        .to_string();

        println!("{}", str);
    }

    #[test]
    fn context_test() {
        let global_variables = vec![GlobalVariableContext {
            is_private: false,
            is_constant: false,
            variable_name: "a".to_string(),
            value: Value {
                context: "12".to_string(),
                value_type: ValueType::Other,
            },
            value_type: LLVMTypes::Int8,
        }];

        let source_filename = "test.zx".to_string();

        let llvm_ir = LLVMContext {
            source_filename,
            global_variables,
            named_metadata: vec![]
        }
        .to_string();

        println!("{}", llvm_ir);
    }

    #[test]
    fn builder_global_var_string_test() {
        let mut builder = LLVMBuilder::new("test.zx");
        let value = String::from("你好");

        builder.crate_global_var(
            "abc".to_string(),
            LLVMTypes::String { len: value.len() },
            value,
            false,
            true,
        );

        let llvm_ir = builder.to_string();
        println!("{}", llvm_ir);
    }

    #[test]
    fn builder_global_var_int_test() {
        let mut builder = LLVMBuilder::new("test.zx");
        let value = String::from("123");

        builder.crate_global_var("abc".to_string(), LLVMTypes::Int8, value, false, false);

        builder.add_named_mata("llvm.ident".to_string(), vec![
            create_number("0".to_string())
        ]);
        builder.add_named_mata("0".to_string(), vec![
            create_ref_string("zx version 1".to_string())
        ]);

        let llvm_ir = builder.to_string();
        println!("{}", llvm_ir);
    }
}
