use std::str::Chars;

#[derive(Debug, Clone)]
pub struct FileStream<'a> {
    code_string_iterators: Chars<'a>,
    currently: char,
    index: usize,
    pub(crate) is_eof: bool
}

impl FileStream <'_> {
    pub fn new(s: &str) -> FileStream {
        let mut chars = s.chars();
        let next_char = chars.next();

        FileStream {
            code_string_iterators: chars,
            currently: next_char.unwrap_or(' '),
            index: 0,
            is_eof: next_char == None
        }
    }

    pub fn get_currently(&self) -> char {
        self.currently
    }

    pub fn next(&mut self) {
        let next_char = self.code_string_iterators.next();
        self.is_eof = next_char == None;
        self.index += 1;

        if !self.is_eof {
            self.currently = next_char.unwrap();
        }
    }
}
