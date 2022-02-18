pub enum Chars {
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
    MoreToken
}

impl Chars {
    fn value(&self) -> &str {
        match *self {
            Chars::CommaToken => ",",
            Chars::ColonToken => ":",
            Chars::LeftCurlyBracketsToken => "{",
            Chars::RightCurlyBracketsToken => "}",
            Chars::LeftSquareBracketsToken => "[",
            Chars::RightSquareBracketsToken => "]",
            Chars::DoubleQuotesToken => "\"",
            Chars::SingleQuotesToken => "'",
            Chars::LeftParenthesesToken => "(",
            Chars::RightParenthesesToken => ")",
            Chars::SlashToken => "/",
            Chars::BackslashToken => "\\",
            Chars::PlusToken => "+",
            Chars::MinusToken => "-",
            Chars::MultiplyToken => "*",
            Chars::EqualToken => "=",
            Chars::DotToken => ".",
            Chars::LessToken => "<",
            Chars::MoreToken => ">",
            _ => ""
        }
    }
}