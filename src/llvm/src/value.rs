use std::fmt;
use std::fmt::Formatter;

pub struct Value {
    pub context: String,
    pub is_string: bool,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value_string = if self.is_string {
            format!(r#"c"{}\00""#, &self.context)
        } else {
            self.context.clone()
        };
        write!(f, "{}", value_string)
    }
}

pub fn create_string(value: String) -> Value {
    Value {
        context: value,
        is_string: true,
    }
}

pub fn create_int(value: String) -> Value {
    Value {
        context: value,
        is_string: false,
    }
}
