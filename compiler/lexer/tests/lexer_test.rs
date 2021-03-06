#[cfg(test)]
mod lexer_test {
    use lexer::Lexer;
    use std::fs;
    use util::report::{Level, Report};
    use util::token::{Literal, Tokens};

    #[test]
    fn test_lexer() {
        let path = "./test_data/test_data.zx".to_string();
        let source = fs::read_to_string(&path).expect("Something went wrong reading the file");
        let mut lexer = Lexer::new(&source);
        match lexer.lexer() {
            Err(error) => {
                Report {
                    level: Level::Error,
                    error,
                }
                .print(&source, &path);
            }
            Ok(()) => {}
        };
        for token in lexer.tokens {
            match token.token_type {
                Tokens::LiteralToken { kid, literal } => match kid {
                    Literal::String => println!("string content: {}", literal),
                    Literal::Float => println!("float content: {}", literal),
                    Literal::PositiveInteger => println!("integer content: {}", literal),
                    Literal::Char => println!("char content: {}", literal),
                    _ => {}
                },
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
    use lexer::file_stream::StringStream;

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

        assert_eq!('a', file_stream.get_currently());
        file_stream.next();
        assert_eq!('b', file_stream.get_currently());
        assert_eq!('c', file_stream.first());
        assert_eq!('b', file_stream.get_currently());
        file_stream.next();
        assert_eq!('c', file_stream.get_currently());
    }
}
