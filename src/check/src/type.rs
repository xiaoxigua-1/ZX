use util::token::Token;

#[derive(Clone)]
pub enum ZXTyped {
    String,
    Integer,
    Char,
    Float,
    Other {
        type_name: Token,
    },
    Void
}