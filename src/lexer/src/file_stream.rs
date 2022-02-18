use std::borrow::Borrow;
use std::fs::File;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct FileStream<'a> {
    code_string_length: usize,
    code_string_iterators: Chars<'a>,
    currently: char,
    index: usize,
    is_eof: bool
}

impl FileStream <'_> {
    pub fn new(s: &str) -> FileStream {
        let mut chars = s.chars();
        let next_char = chars.next().unwrap();

        FileStream {
            code_string_length: s.len(),
            code_string_iterators: chars,
            currently: next_char,
            index: 0,
            is_eof: false
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

#[cfg(test)]
mod file_stream_test {
    use crate::FileStream;

    #[test]
    fn test_file_stream() {
        let mut file_stream = FileStream::new("abc");

        loop {

            if file_stream.is_eof {
                break;
            }
            println!("{}", file_stream.get_currently());
            file_stream.next();
        }
    }
}