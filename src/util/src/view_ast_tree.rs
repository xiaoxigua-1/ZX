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
                println!("{}├── Block", self.line_start(index));
                statements.iter().for_each(|statement| { self.statement(index + 1, statement)})
            }
            VariableDeclaration { var_name, type_identifier, value, .. } =>
                self.variable_declaration(index, var_name, type_identifier, value),
            Statement::Expression { expression } => self.expression(expression, index),
            If { condition, else_statement, block, .. } => self.if_statement(index, else_statement, block, condition),
            Else { next, .. } => self.else_statemtnt(index, next),
            WhileLoop { block, condition, .. } => self.while_loop(index, block, condition),
            ForLoop { block, iter, for_var_name, .. } => self.for_loop(index, for_var_name, iter, block),
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
        self.statement(index + 1, block)
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
            Value { kid, next, content } => {
                println!("{}├── Type {:?}", line_start, kid);
                println!("{}├── Value {}", line_start, self.literal(content));
                if let Some(next) = &**next {
                    println!("{}├── Next", line_start);
                    self.expression(&next, index + 1);
                }
            }
            Call { call_name, arguments, .. } => {
                println!("{line_start}├── Call {}", self.literal(call_name));
                arguments.into_iter().for_each(|argument| {
                    println!("{line_start}|    ├── arg");
                    self.expression(argument, index + 2);
                })
            }
            SubMember { sub_member } => {
                println!("{line_start}├── SubMember");
                self.expression(&*sub_member, index + 1);
            }
            Identifier { identifier, next } => {
                println!("{line_start}├── Identifier {}", self.literal(identifier));
                if let Some(next) = next {
                    println!("{line_start}|    ├── next");
                    self.expression(next, index + 2);
                }
            }
            _ => {}
        }
    }

    fn variable_declaration(&self, index: i32, variable_name: &Token, type_identifier: &Option<Expression>, value: &Option<Box<Statement>>) {
        let line_start = self.line_start(index);
        println!("{}├── variable {}", line_start, self.literal(variable_name));
        if let Some(expression) = type_identifier {
            self.expression(expression, index + 1);
        }
        if let Some(value) = value {
            self.statement(index + 1, value);
        }
    }

    fn if_statement(&self, index: i32, else_statement: &Box<Option<Statement>>, block: &Box<Statement>, condition: &Expression) {
        let line_start = self.line_start(index);
        println!("{line_start}├── if statement");
        println!("{line_start}|    ├── condition");
        self.expression(condition, index + 2);
        self.statement(index + 1, &*block);
        if let Some(else_statement) = &**else_statement {
            self.statement(index + 1, &else_statement);
        }
    }

    fn else_statemtnt(&self, index: i32, next: &Box<Option<Statement>>) {
        let line_start = self.line_start(index);
        println!("{line_start}├── else statement");
         if let Some(next) = &**next {
            self.statement(index + 1, &next);
        }
    }

    fn while_loop(&self, index: i32, block: &Box<Statement>, condition: &Expression) {
        let line_start = self.line_start(index);
        println!("{line_start}├── While loop");
        self.expression(condition, index + 1);
        self.statement(index + 1, &**block);
    }

    fn for_loop(&self, index: i32, item_name: &Token, iter: &Box<Statement>, block: &Box<Statement>) {
        let line_start = self.line_start(index);
        println!("{line_start}├── For loop");
        println!("{line_start}|    ├── Item name `{}`", self.literal(item_name));
        println!("{line_start}|    ├── iter");
        self.statement(index + 2, &**iter);
        self.statement(index + 1, block);
    }

    fn literal(&self, token: &Token) -> String {
        match token.token_type {
            Tokens::IdentifierToken { ref literal } => literal.to_string(),
            Tokens::LiteralToken { ref literal, .. } => literal.to_string(),
            _ => String::new()
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