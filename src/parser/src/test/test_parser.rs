#[cfg(test)]
mod test_parser {
    use crate::Parser;
    use lexer::Lexer;
    use std::fs;
    use util::repost::{Level, Report};
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
        let paths = fs::read_dir("./test_data").unwrap();
        paths.into_iter().for_each(|dir| {
            let path = dir.unwrap().path().display().to_string();
            println!("Test {path} file");
            let source = fs::read_to_string(&path).expect("Something went wrong reading the file");
            let mut lexer = Lexer::new(&source);

            match lexer.lexer() {
                Ok(()) => {
                    let mut parser = Parser::new(&lexer.tokens);
                    parser.parse(&path, &source);
                    ViewASTTree { ast_tree: parser.asts }.main();
                }
                Err(error) => {
                    Report { level: Level::Error, error }.print(&source, &path);
                }
            }
        });

    }
}
