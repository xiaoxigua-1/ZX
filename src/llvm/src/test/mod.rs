#[cfg(test)]
mod context_test {
    use crate::builder::LLVMBuilder;
    use crate::context::{GlobalVariableContext, LLVMContext};
    use crate::llvm_type::LLVMTypes;


    #[test]
    fn global_variable_context() {
        let str = GlobalVariableContext {
            is_constant: false,
            variable_name: "a".to_string(),
            value: "123".to_string(),
            value_type: LLVMTypes::Int8
        }.to_string();

        println!("{}", str);
    }

    #[test]
    fn context_test() {
        let global_variables = vec![
            GlobalVariableContext {
                is_constant: false,
                variable_name: "a".to_string(),
                value: "123".to_string(),
                value_type: LLVMTypes::Int8
            }
        ];

        let source_filename = "test.zx".to_string();

        let llvm_ir = LLVMContext {
            source_filename,
            global_variables
        }.to_string();

        println!("{}", llvm_ir);
    }

    #[test]
    fn builder_test() {
        let mut builder = LLVMBuilder::new("test.zx");
        builder.crate_global_var(
            "abc".to_string(),
            LLVMTypes::Int8,
            "2".to_string(),
            false
        );

        let llvm_ir = builder.to_string();
        println!("{}", llvm_ir);
    }
}