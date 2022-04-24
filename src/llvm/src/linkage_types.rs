use std::fmt;
use std::fmt::Formatter;
use LinkageTypes::*;

pub enum LinkageTypes {
    Private,
    Internal,
    AvailableExternally,
    Linkonce,
    Weak,
    Common,
    Appending,
    ExternWeak,
    WeakOdr,
    LinkonceOdr,
    External,
    NULL,
}

impl fmt::Display for LinkageTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Private => " private",
            Internal => " internal",
            AvailableExternally => " available_externally",
            Linkonce => " linkonce",
            Weak => " weak",
            Common => " common",
            Appending => " appending",
            ExternWeak => " external_weak",
            WeakOdr => " weak_odr",
            LinkonceOdr => " linkonce_odr",
            External => " external",
            NULL => "",
        })
    }
}