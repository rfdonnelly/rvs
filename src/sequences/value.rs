use super::Sequence;

pub struct Value {
    last: u32,
    done: bool,
}

impl Value {
    pub fn new(value: u32) -> Value {
        Value {
            last: value,
            done: false,
        }
    }
}

impl Sequence for Value {
    fn next(&mut self) -> u32 {
        self.done = true;

        self.last
    }

    fn last(&self) -> u32 {
        self.last
    }

    fn done(&self) -> bool {
        self.done
    }
}
