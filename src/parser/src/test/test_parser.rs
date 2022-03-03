#[cfg(test)]
mod test_parser {
    use crate::Parser;
    use lexer::Lexer;
    use std::fs;
    use util::repost::{Level, Repost};
    use util::token::{Literal, Position, Token, Tokens};
    use util::view_ast_tree::ViewASTTree;

    #[test]
    fn test_parser() {
        let pos = Position { start: 0, end: 0 };

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
        Parser::new(&tokens);
    }

    #[test]
    fn test_parser_function() {
        let path = "./test_data/function.zx".to_string();
        let source = fs::read_to_string(&path).expect("Something went wrong reading the file");
        let mut lexer = Lexer::new(&source);
        match lexer.lexer() {
            Ok(()) => {
                let mut parser = Parser::new(&lexer.tokens);
                parser.parse(&path, &source);
                ViewASTTree { ast_tree: parser.asts }.main();
            }
            Err(error) => {

                Repost { level: Level::Error, error }.print(&source, &path);
            }
        }
    }
}
