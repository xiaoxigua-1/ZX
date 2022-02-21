#[derive(Display, Debug, Clone)]
pub enum Tokens {
    // End of file
    EOF,
    // `\n`
    LineSeparatorToken,
    // `,`
    CommaToken,
    // `:`
    ColonToken,
    // `{`
    LeftCurlyBracketsToken,
    // `}`
    RightCurlyBracketsToken,
    // `[`
    LeftSquareBracketsToken,
    // `]`
    RightSquareBracketsToken,
    // `(`
    LeftParenthesesToken,
    // `)`
    RightParenthesesToken,
    // `/`
    SlashToken,
    // `\`
    BackslashToken,
    // `+`
    PlusToken,
    // `-`
    MinusToken,
    // `*`
    MultiplyToken,
    // `=`
    EqualToken,
    // `.`
    DotToken,
    // `<`
    LessToken,
    // `>`
    MoreToken,
    // `;`
    SemicolonToken,
    // `!`
    ExclamationToken,
    IdentifierToken {
        literal: String
    },
    LiteralToken {
        kid: Literal,
        literal: String
    }
}

#[derive(Display, Debug, Clone, EnumString)]
pub enum Literal {
    String,
    Integer,
    Float,
    Char
}

#[derive(Debug, Clone)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: Tokens,
    pub pos: Position,
}

impl Token {
    pub fn is_token_type(&self, token_type: Tokens) -> bool {
        token_type.to_string() == self.token_type.to_string()
    }

    pub fn is_token_type_str(&self, token_type: &str) -> bool {
        self.token_type.to_string() == token_type
    }
}