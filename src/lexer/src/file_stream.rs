use std::str::Chars;

#[derive(Debug, Clone)]
pub struct StringStream<'a> {
    code_string_iterators: Chars<'a>,
    currently: char,
    prev: char,
    pub(crate) index: usize,
    pub(crate) is_eof: bool,
}

impl StringStream<'_> {
    pub fn new(s: &str) -> StringStream {
        let mut chars = s.chars();
        let next_char = chars.next();

        StringStream {
            code_string_iterators: chars,
            currently: next_char.unwrap_or(' '),
            prev: '\0',
            index: 0,
            is_eof: next_char == None,
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
            self.prev = next_char.unwrap();
            self.currently = next_char.unwrap();
        }
    }

    pub fn first(&self) -> char {
        self.code_string_iterators.clone().next().unwrap_or('\0')
    }
}
