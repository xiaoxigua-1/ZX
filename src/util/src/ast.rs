use crate::token::{Literal, Token};

#[derive(Debug)]
pub enum Statement {
    Static {
        statement: Box<Statement>
    },
    Public {
        statement: Box<Statement>
    },
    FunctionDeclaration {
        fn_keyword: Token,
        function_name: Token,
        left_parentheses: Token,
        parameters: Vec<Parameter>,
        right_parentheses: Token,
        return_type: Option<Expression>,
        block: Box<Statement>,
    },
    VariableDeclaration {
        var_keyword: Token,
        var_name: Token,
        colon: Option<Token>,
        type_identifier: Option<Expression>,
        equal: Option<Token>,
        value: Option<Expression>,
    },
    Import {
        im_keyword: Token,
        // idk
    },
    Class {},
    If {
        if_keyword: Token,
        condition: Expression,
        block: Box<Statement>,
        else_statement: Box<Option<Statement>>,
    },
    Else {
        else_keyword: Token,
        next: Box<Option<Statement>>,
    },
    WhileLoop {},
    ForLoop {},
    Return {
        return_keyword: Token,
        return_expression: Expression,
    },
    Block {
        left_curly_brackets: Token,
        statements: Vec<Statement>,
        right_curly_brackets: Token,
    },
    Expression {
        expression: Expression,
    },
}

#[derive(Debug)]
pub enum Expression {
    Call {
        call_name: Token,
        left_parentheses: Token,
        arguments: Vec<Expression>,
        right_parentheses: Token,
        next: Box<Option<Expression>>,
    },
    Value {
        kid: Literal,
        content: Token,
        next: Box<Option<Expression>>,
    },
    Path {
        identifier: Token,
        next: Box<Expression>,
    },
    SubMember {
        sub_member: Box<Expression>
    },
    Type {
        identifier: Token,
        nullable: bool,
    },
    Identifier {
        identifier: Token,
        next: Box<Expression>,
    },
}

#[derive(Debug)]
pub struct Parameter {
    pub parameter_name: Token,
    pub type_expression: Expression,
}