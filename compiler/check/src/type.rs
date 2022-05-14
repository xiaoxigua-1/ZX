#[derive(Clone, Debug, PartialEq)]
pub enum ZXTyped {
    String { nullable: bool },
    Integer { nullable: bool },
    Char { nullable: bool },
    Float { nullable: bool },
    Other { type_name: String },
    Void,
}
