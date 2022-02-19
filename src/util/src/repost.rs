pub enum Level {
    Error,
    Warning,
    Debug
}

pub struct Repost {
    pub level: Level,
    pub message: String,
}

impl Repost {
    pub fn print(&self) {
        let mut s = match self.level {
            Level::Error => "\x1b[31m".to_string(),
            Level::Warning => "\x1b[33m".to_string(),
            Level::Debug => "\x1b[34m".to_string(),
        };

        s.push_str(&self.message);
        s.push_str("\x1b[0m");
        println!("{}", s);
    }
}