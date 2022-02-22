use crate::token::Token;

pub enum Statement {
    StaticFunctionDeclaration {
        static_keyword: Token,
        fn_keyword: Token,
        function_name: Token,
        left_parentheses: Token,
        parameters: Vec<Parameter>,
        right_parentheses: Token,
        left_curly_brackets: Token,
        statement: Vec<Statement>,
        right_curly_brackets: Token,
    },
    PublicFunctionDeclaration {
        pub_keyword: Option<Token>,
        fn_keyword: Token,
        function_name: Token,
        left_parentheses: Token,
        parameters: Vec<Parameter>,
        right_parentheses: Token,
        left_curly_brackets: Token,
        statement: Vec<Statement>,
        right_curly_brackets: Token,
    },
    FunctionDeclaration {
        fn_keyword: Token,
        function_name: Token,
        left_parentheses: Token,
        parameters: Vec<Parameter>,
        right_parentheses: Token,
        left_curly_brackets: Token,
        statement: Vec<Statement>,
        right_curly_brackets: Token,
    },
    VariableDeclaration {
        var_keyword: Token,
        colon: Option<Token>,
        type_identifier: Option<Token>,
        equal: Option<Token>,
        value: Option<Vec<Expression>>,
    },
    Import {
        im_keyword: Token,
        // idk
    },
    Class {},
    If {},
    Else {},
    WhileLoop {},
    ForLoop {},
}

pub enum Expression {
    Call {
        call_name: Token,
        left_parentheses: Token,
        arguments: Vec<Expression>,
        right_parentheses: Token,
        next: Box<Expression>
    },
    Value {},
    Path {
        identifier: Token,
        next: Box<Expression>
    }
}

pub struct Parameter {
    parameter_name: Token,
    colon: Token,
    type_identifier: Token,
}