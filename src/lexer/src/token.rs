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
    IdentifierToken
}

#[derive(Debug)]
pub struct Token {
    pub(crate) token_type: Tokens,
    pub(crate) literal: String,
}