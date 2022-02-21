use crate::token::Token;

pub enum Statement {
    StaticFunctionDeclaration {
        static_keyword: Token,
        fn_keyword: Token,
        left_parentheses: Token,
        parameters: Vec<Parameter>,
        right_parentheses: Token,
    },
    PublicFunctionDeclaration {
        pub_keyword: Option<Token>,
        fn_keyword: Token,
        left_parentheses: Token,
        parameters: Vec<Parameter>,
        right_parentheses: Token,
    },
    FunctionDeclaration {
        fn_keyword: Token,
        left_parentheses: Token,
        parameters: Vec<Parameter>,
        right_parentheses: Token,
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

    },
    Class {},
    If {},
    Else {},
    WhileLoop {},
    ForLoop {},
}

pub enum Expression {
    CallFunction {
        function_name: Token,
        left_parentheses: Token,
    },
    Value {},
}

pub struct Parameter {
    parameter_name: Token,
    colon: Token,
    type_identifier: Token,
}