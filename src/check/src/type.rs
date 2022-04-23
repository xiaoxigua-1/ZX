use util::token::Token;

#[derive(Clone, Debug)]
pub enum ZXTyped {
    String,
    Integer,
    Char,
    Float,
    Other { type_name: Token },
    Void,
}
