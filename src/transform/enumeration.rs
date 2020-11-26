use indexmap::IndexMap;

#[derive(Debug)]
pub struct Enum {
    pub items: IndexMap<String, u32>,
}

impl Enum {
    pub fn new(items: IndexMap<String, u32>) -> Enum {
        Enum { items }
    }
}
