use util::token::Position;

pub struct LexerError {
    pub(crate) message: String,
    pub(crate) pos: Position,
}