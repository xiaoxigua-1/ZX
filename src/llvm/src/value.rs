use std::fmt;
use std::fmt::Formatter;

pub struct Value {
    pub context: String,
    pub value_type: ValueType,
}

pub enum ValueType {
    String,
    RefString,
    Other,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value_string = match self.value_type {
            ValueType::String => {
                if self.context.is_empty() {
                    String::from("zeroinitializer")
                } else {
                    format!(r#"c"{}\00""#, &self.context)
                }
            }
            ValueType::RefString => format!(r#""{}""#, &self.context),
            ValueType::Other => self.context.clone(),
        };

        write!(f, "{}", value_string)
    }
}

/// create string
/// Example: `c"abc\00"`
pub fn create_string<T: fmt::Display>(value: T) -> Value {
    Value {
        context: value.to_string(),
        value_type: ValueType::String,
    }
}

/// create reference string
/// Example: `"abc"`
pub fn create_ref_string<T: fmt::Display>(value: T) -> Value {
    Value {
        context: value.to_string(),
        value_type: ValueType::RefString,
    }
}

/// create number
/// Example: `123`
pub fn create_number<T: fmt::Display>(value: T) -> Value {
    Value {
        context: value.to_string(),
        value_type: ValueType::Other,
    }
}

/// create local variable
/// Example: `%1`
pub fn create_local_variable<T: fmt::Display>(name: T) -> Value {
    Value {
        context: format!("%{}", name),
        value_type: ValueType::Other,
    }
}

/// create global variable
/// Example: `@abc`
pub fn create_global_variable<T: fmt::Display>(name: T) -> Value {
    Value {
        context: format!("@{}", name),
        value_type: ValueType::Other,
    }
}

pub fn create_void() -> Value {
    Value {
        context: String::from("void"),
        value_type: ValueType::Other,
    }
}
