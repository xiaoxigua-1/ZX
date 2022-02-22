use crate::error::ZXError;
use crate::token::Position;
use std::fs;
use std::path::PathBuf;

pub enum Level {
    Error,
    Warning,
    Debug,
}

#[derive(Debug)]
struct PrintSource {
    source: String,
    line_number: i32,
    arrow_position: Position,
}

pub struct Repost {
    pub level: Level,
    pub error: ZXError
}

impl Repost {
    pub fn print(&self, source: &String, path: &String) {
        let mut source_index: usize = 0;
        let mut line_number = 1;
        let mut print_source = vec![];
        let color_char = match self.level {
            Level::Error => "\x1b[31m".to_string(),
            Level::Warning => "\x1b[33m".to_string(),
            Level::Debug => "\x1b[34m".to_string(),
        };
        let (message, pos) = match &self.error {
            ZXError::SyntaxError { message, pos } => (message, pos),
            ZXError::NameError { message, pos } => (message, pos),
            ZXError::NullError { message, pos } => (message, pos),
            ZXError::TypeError { message, pos } => (message, pos),
        };

        self.print_error_message(color_char, message.to_string());

        for line_code in source.split('\n') {
            if source_index + line_code.len() >= pos.start {
                print_source.push(PrintSource {
                    source: line_code.to_string(),
                    line_number,
                    arrow_position: Position {
                        start: pos.start - source_index,
                        end: if source_index + line_code.len() > pos.end {
                            pos.end - source_index
                        } else {
                            source_index + line_code.len()
                        },
                    },
                });

                if source_index + line_code.len() > pos.end {
                    break;
                }
            }

            line_number += 1;
            source_index += line_code.len() + 1;
        }

        let max_number = print_source
            .iter()
            .max_by(|a, b| a.line_number.cmp(&b.line_number))
            .unwrap();
        let max_number = format!("{}", max_number.line_number).len() + 1;
        let srcdir = PathBuf::from(path);
        let path_string = fs::canonicalize(&srcdir).unwrap().into_os_string().into_string().unwrap();

        for source in print_source {
            println!(" ===> {}:{}:{}", path_string, source.line_number, source.arrow_position.start + 1);
            println!("{:<width$}|", "", width = max_number);
            println!("{:<width$}| {}", source.line_number, source.source, width = max_number);
            println!("{space:<width$}| {space:>arrow_start$}{space:^>arrow_width$}",
                space = "",
                width = max_number,
                arrow_start = source.arrow_position.start,
                arrow_width = source.arrow_position.end - source.arrow_position.start + 1
            );
            println!("{:<width$}|", "", width = max_number);
        }
    }

    fn print_error_message(&self, color_char: String, message: String) {
        println!("{}{:?}: {}\x1b[0m", color_char, self.error.to_string(), message);
    }
}