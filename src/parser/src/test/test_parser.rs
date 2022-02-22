#[cfg(test)]
mod test_parser {
    use crate::Parser;
    use util::token::{Token, Position, Tokens, Literal};
    use lexer::Lexer;

    #[test]
    fn test_parser() {
        let pos = Position {
            start: 0,
            end: 0,
        };

        let tokens = vec![
            Token {
                token_type: Tokens::LiteralToken {
                    kid: Literal::String,
                    literal: "abc".to_string(),
                },
                pos: pos.clone(),
            },
            Token {
                token_type: Tokens::EOF,
                pos: pos.clone(),
            },
        ];
        let mut parser = Parser::new(&tokens);
        parser.parse();
    }

    #[test]
    fn test_parser_function() {
        let mut lexer = Lexer::new(&"./test_data/function.zx".to_string());
        if let Ok(()) = lexer.lexer() {
            let mut parser = Parser::new(&lexer.tokens);
            parser.parse();
        }
    }
}