use std::str::Chars;

#[derive(Debug, Clone)]
pub struct StringStream<'a> {
    code_string_iterators: Chars<'a>,
    currently: char,
    back_list: Vec<char>,
    back_index: usize,
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
            index: 0,
            back_index: 0,
            back_list: vec![next_char.unwrap_or(' ')],
            is_eof: next_char == None,
        }
    }

    pub fn get_currently(&self) -> char {
        self.currently
    }

    pub fn next(&mut self) {
        if self.back_index > self.index {
            self.currently = self.back_list.pop().unwrap();
            self.index += 1;
        } else {
            let next_char = self.code_string_iterators.next();
            self.is_eof = next_char == None;
            self.index += 1;
            self.back_index += 1;

            if !self.is_eof {
                self.back_list.push(next_char.unwrap());
                self.currently = next_char.unwrap();
            }
        }
    }

    pub fn back(&mut self) {
        let back_char = self.back_list.get(self.back_index - self.index);
        self.index -= 1;
        self.currently = *back_char.unwrap();
    }
}
