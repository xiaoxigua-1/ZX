#[cfg(test)]
mod lexer_test {
    use util::token::Tokens;
    use crate::Lexer;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new(&"./test_data/test_data.zx".to_string());
        match lexer.lexer() {
            Err(()) => {}
            Ok(()) => {}
        };
        for token in lexer.tokens {
            match token.token_type {
                Tokens::LiteralToken { kid: _, literal} => {
                    println!("string content: {}", literal);
                }
                Tokens::IdentifierToken { literal } => {
                    println!("identifier content: {}", literal);
                }
                _ => {
                    println!("{:#?}", token.token_type);
                }
            }
        }
    }
}

#[cfg(test)]
mod file_stream_test {
    use crate::StringStream;

    #[test]
    fn test_file_stream() {
        let mut file_stream = StringStream::new("abc");

        while !file_stream.is_eof {
            println!("{}", file_stream.get_currently());
            file_stream.next();
        }
    }
}