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
    DoubleQuotesToken,
    SingleQuotesToken,
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
    ArrowToken,
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
    Char
}

#[derive(Debug)]
pub struct Position {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

#[derive(Debug)]
pub struct Token {
    pub(crate) token_type: Tokens,
    pub(crate) pos: Position,
}