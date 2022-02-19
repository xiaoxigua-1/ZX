#[cfg(test)]
mod lexer_test {
    use crate::Lexer;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("./test_data/test_data.zx".to_string());
        lexer.lexer();
        println!("{:?}", lexer.get_tokens())
    }
}

#[cfg(test)]
mod file_stream_test {
    use crate::FileStream;

    #[test]
    fn test_file_stream() {
        let mut file_stream = FileStream::new("abc");

        while !file_stream.is_eof {
            println!("{}", file_stream.get_currently());
            file_stream.next();
        }
    }
}