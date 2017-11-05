use super::Sequence;

pub struct Value {
    prev: u32,
    done: bool,
}

impl Value {
    pub fn new(value: u32) -> Value {
        Value {
            prev: value,
            done: false,
        }
    }
}

impl Sequence for Value {
    fn next(&mut self) -> u32 {
        self.done = true;

        self.prev
    }

    fn prev(&self) -> u32 {
        self.prev
    }

    fn done(&self) -> bool {
        self.done
    }
}
