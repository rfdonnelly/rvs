use super::Sequence;

pub struct Value {
    last: u32,
}

impl Value {
    pub fn new(value: u32) -> Value {
        Value {
            last: value,
        }
    }
}

impl Sequence for Value {
    fn next(&mut self) -> u32 {
        self.last
    }

    fn last(&self) -> u32 {
        self.last
    }
}
