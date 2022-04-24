use std::fmt;
use std::fmt::Formatter;

pub struct Value {
    pub context: String,
    pub value_type: ValueType,
}

pub enum ValueType {
    String,
    Other,
    RefString,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value_string = match self.value_type {
            ValueType::String => if self.context.is_empty() {
                String::from("zeroinitializer")
            } else {
                format!(r#"c"{}\00""#, &self.context)
            },
            ValueType::RefString => format!(r#""{}""#, &self.context),
            ValueType::Other => self.context.clone()
        };

        write!(f, "{}", value_string)
    }
}

pub fn create_string(value: String) -> Value {
    Value {
        context: value,
        value_type: ValueType::String,
    }
}

pub fn create_ref_string(value: String) -> Value {
    Value {
        context: value,
        value_type: ValueType::RefString,
    }
}

pub fn create_number(value: String) -> Value {
    Value {
        context: value,
        value_type: ValueType::Other,
    }
}
