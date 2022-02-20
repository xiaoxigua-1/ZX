#[cfg(test)]
mod lexer_test {
    use util::token::{Literal, Tokens};
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
                Tokens::LiteralToken { kid, literal } => {
                    match kid {
                        Literal::String => println!("string content: {}", literal),
                        Literal::Float => println!("float content: {}", literal),
                        Literal::Integer => println!("integer content: {}", literal),
                        Literal::Char => println!("char content: {}", literal),
                    }
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

    #[test]
    fn test_file_stream_back() {
        let mut file_stream = StringStream::new("abcd");

        println!("{}", file_stream.get_currently()); // a
        file_stream.next();
        println!("{}", file_stream.get_currently()); // b
        file_stream.back();
        println!("{}", file_stream.get_currently()); // a
        file_stream.next();
        println!("{}", file_stream.get_currently()); // b
        file_stream.next();
        println!("{}", file_stream.get_currently()); // c
    }
}