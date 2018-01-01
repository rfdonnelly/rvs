use linked_hash_map::LinkedHashMap;

pub struct Enum {
    pub items: LinkedHashMap<String, u32>,
}

impl Enum {
    pub fn new(items: LinkedHashMap<String, u32>) -> Enum {
        Enum {
            items,
        }
    }
}
