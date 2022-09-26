use inkwell::types::StructType;

pub struct Structs<'a> {
    structs: Vec<(String, StructType<'a>)>
}

impl<'a> Structs<'a> {
    pub fn new() -> Structs<'static> {
        Structs {
            structs: Vec::new()
        }
    }

    pub fn find(&self, name: &String) -> StructType<'a> {
        self.structs.iter().find(|value| {
            value.0.eq(name)
        }).unwrap().1
    }
}