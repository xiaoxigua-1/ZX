use crate::ast::Statement::*;
use crate::ast::Expression::*;
use crate::ast::{Parameter, Statement};
use crate::token::{Token, Tokens};
use crate::ast::Expression;

pub struct ViewASTTree {
    pub ast_tree: Vec<Statement>,
}

impl ViewASTTree {
    pub fn main(&self) {
        for statement in self.ast_tree.iter() {
            self.statement(0, statement)
        }
    }

    fn statement(&self, index: i32, statement: &Statement) {
        match statement {
            FunctionDeclaration { function_name, parameters, return_type, block, .. } =>
                self.function_declaration(function_name, parameters, return_type, block, index),
            Block { statements, .. } => {
                statements.iter().for_each(|statement| { self.statement(index, statement)})
            }
            _ => {}
        }
    }

    fn line_start(&self, index: i32) -> String {
        let mut line_start = String::new();
        (0..index).into_iter().for_each(|_| { line_start.push_str("|    ") });
        line_start
    }

    fn function_declaration(&self, function_name: &Token, parameters: &Vec<Parameter>, return_type: &Option<Expression>, block: &Box<Statement>, index: i32) {
        let line_start = self.line_start(index);
        println!("{line_start}├── Function {}", self.literal(function_name));
        self.function_parameters(parameters, index + 1);
        if let Some(type_expression) = return_type {
            println!("{line_start}|    ├── Return Type");
            self.expression(type_expression, index + 1);
        }
        println!("{line_start}|    ├── Statements");
        self.statement(index + 2, block)
    }

    fn function_parameters(&self, parameters: &Vec<Parameter>, index: i32) {
        let line_start = self.line_start(index);
        if !parameters.is_empty() {
            println!("{line_start}├── Parameters");
            parameters.iter().for_each(|parameter| {
                println!("{line_start}|    ├── {}", self.literal(&parameter.parameter_name));
            })
        }
    }

    fn expression(&self, expression: &Expression, index: i32) {
        let line_start = self.line_start(index);
        match expression {
            Type { identifier, nullable } => {
                println!("{line_start}|    ├── {} type", self.literal(identifier));
                println!("{line_start}|    └── nullable {}", nullable);
            }
            _ => {}
        }
    }

    fn literal(&self, token: &Token) -> String {
        if let Tokens::IdentifierToken { ref literal } = token.token_type {
            literal.to_string()
        } else {
            String::new()
        }
    }
}
/*
├── if
|   ├── fn keyword
|   ├── left_parentheses
|   ├──
|
└──
 */

#[cfg(test)]
mod token_tree_test {
    use crate::ast::Statement;
    use crate::token::{Position, Token, Tokens};
    use crate::view_ast_tree::ViewASTTree;

    const POS: Position = Position {
        start: 0,
        end: 0,
    };
    const TOKEN: Token = Token {
        token_type: Tokens::EOF,
        pos: POS
    };

    #[test]
    fn test() {
        let ast = vec![
            Statement::FunctionDeclaration {
                fn_keyword: TOKEN,
                function_name: TOKEN,
                left_parentheses: TOKEN,
                parameters: vec![],
                right_parentheses: TOKEN,
                return_type: None,
                block: Box::new(Statement::Block {
                    left_curly_brackets: TOKEN,
                    statements: vec![],
                    right_curly_brackets: TOKEN
                })
            }
        ];
        let view = ViewASTTree { ast_tree: ast };
        view.main();
    }
}