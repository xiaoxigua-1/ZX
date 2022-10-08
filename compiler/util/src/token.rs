use crate::error::ZXError;

#[derive(Display, Debug, Clone)]
pub enum Tokens {
    /// End of file
    EOF,
    /// `\n`
    LineSeparatorToken,
    /// `,`
    CommaToken,
    /// `:`
    ColonToken,
    /// `{`
    LeftCurlyBracketsToken,
    /// `}`
    RightCurlyBracketsToken,
    /// `[`
    LeftSquareBracketsToken,
    /// `]`
    RightSquareBracketsToken,
    /// `(`
    LeftParenthesesToken,
    /// `)`
    RightParenthesesToken,
    /// `/`
    SlashToken,
    /// `\`
    BackslashToken,
    /// `+`
    PlusToken,
    /// `-`
    MinusToken,
    /// `*`
    MultiplyToken,
    /// `=`
    EqualToken,
    /// `.`
    DotToken,
    /// `<`
    LessToken,
    /// `>`
    MoreToken,
    /// `;`
    SemicolonToken,
    /// `!`
    ExclamationToken,
    /// `?`
    QuestionMarkToken,
    /// `&`
    AmpersandToken,
    /// `%`
    PercentToken,
    /// `$`
    StdToken,
    /// `example`
    IdentifierToken { literal: String },
    ///`'a'` or `"example"` or `123` or `.3` or `0.3`
    LiteralToken { kid: Literal, literal: String },
}

#[derive(Display, Debug, Clone, EnumString)]
pub enum Literal {
    String,
    PositiveInteger,
    Float,
    Char,
    NegativeInteger,
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
    pub fn is_token_type(&self, token_type: &Tokens) -> bool {
        token_type.to_string() == self.token_type.to_string()
    }

    pub fn is_token_type_str(&self, token_type: &str) -> bool {
        self.token_type.to_string() == token_type
    }

    pub fn get_string(&self) -> Result<String, ZXError> {
        match &self.token_type {
            Tokens::IdentifierToken { literal } => Ok(literal.to_string()),
            Tokens::LiteralToken { literal, .. } => Ok(literal.to_string()),
            _ => Err(ZXError::InternalError {
                message: "Token to string error".into(),
            }),
        }
    }
}
