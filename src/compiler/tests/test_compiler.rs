#[cfg(test)]
mod test_compiler {
    use compiler::Compiler;

    #[test]
    fn test_compiler() {
        Compiler {
            path: "./test_data/test.zx".to_string(),
        }
        .compile()
        .expect("sad");
    }
}
