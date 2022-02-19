use crate::error::ZXError;
use crate::token::Position;


pub enum Level {
    Error,
    Warning,
    Debug
}

pub struct Repost {
    pub level: Level,
    pub error_type: ZXError,
    pub message: String,
    pub pos: Position
}

impl Repost {
    pub fn print(&self) {
        let color_char = match self.level {
            Level::Error => "\x1b[31m".to_string(),
            Level::Warning => "\x1b[33m".to_string(),
            Level::Debug => "\x1b[34m".to_string(),
        };

        println!("{}{:?}: {}\x1b[0m", color_char, self.error_type, self.message);
    }
}