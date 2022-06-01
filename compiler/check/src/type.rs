use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
pub enum ZXTyped {
    String { nullable: bool },
    Integer { nullable: bool },
    Char { nullable: bool },
    Float { nullable: bool },
    Other(String),
    Void,
}

impl fmt::Display for ZXTyped {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ZXTyped::String { .. } => "Str".to_string(),
                ZXTyped::Integer { .. } => "Int".to_string(),
                ZXTyped::Char { .. } => "Char".to_string(),
                ZXTyped::Float { .. } => "Float".to_string(),
                ZXTyped::Other(type_string) => type_string.to_string(),
                ZXTyped::Void => "Void".to_string(),
            }
        )
    }
}
