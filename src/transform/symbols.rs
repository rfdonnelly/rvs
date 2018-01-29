use super::enumeration::Enum;

use std::collections::HashMap;

pub enum Symbol {
    Variable(usize),
    Enum(Enum),
    EnumMember(u32),
}

pub struct Symbols {
    symbols: HashMap<String, Symbol>,
}

impl Symbols {
    pub fn new() -> Symbols {
        Symbols {
            symbols: HashMap::new(),
        }
    }

    pub fn insert_enum(&mut self, name: &str, enumeration: Enum) {
        self.symbols.insert(name.into(), Symbol::Enum(enumeration));
    }

    pub fn insert_enum_member<S>(&mut self, name: S, value: u32)
    where
        S: Into<String>,
    {
        self.symbols.insert(name.into(), Symbol::EnumMember(value));
    }

    pub fn insert_variable(&mut self, name: &str, index: usize) {
        self.symbols.insert(name.into(), Symbol::Variable(index));
    }

    pub fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}
