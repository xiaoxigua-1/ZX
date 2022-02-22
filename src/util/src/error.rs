use crate::token::Position;

#[derive(Debug, Display)]
pub enum ZXError {
    SyntaxError {
        message: String,
        pos: Position
    },
    TypeError {
        message: String,
        pos: Position
    },
    NameError {
        message: String,
        pos: Position
    },
    NullError {
        message: String,
        pos: Position
    },
}