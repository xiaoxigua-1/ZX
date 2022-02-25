use crate::Lexer;

impl Lexer {
    pub(crate) fn escapes(&self, now_char: char) -> char {
        match now_char {
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            '0' => '\0',
            _ => now_char,
        }
    }
}
