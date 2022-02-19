#[derive(Debug)]
pub enum Tokens {
    EOF,
    LineSeparator,
    CommaToken,
    ColonToken,
    LeftCurlyBracketsToken,
    RightCurlyBracketsToken,
    LeftSquareBracketsToken,
    RightSquareBracketsToken,
    LeftParenthesesToken,
    RightParenthesesToken,
    SlashToken,
    BackslashToken,
    PlusToken,
    MinusToken,
    MultiplyToken,
    EqualToken,
    DotToken,
    LessToken,
    MoreToken,
    SemicolonToken,
    ExclamationToken,

    IdentifierToken {
        literal: String
    },
    LiteralToken {
        kid: Literal,
        literal: String
    }
}

#[derive(Debug)]
pub enum Literal {
    String,
    Number,
    Float,
    Char
}

#[derive(Debug)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: Tokens,
    pub pos: Position,
}