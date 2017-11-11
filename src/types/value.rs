use rand::Rng;

use types::Rv;
use types::RvData;

pub struct Value {
    data: RvData,
}

impl Value {
    pub fn new(value: u32) -> Value {
        Value {
            data: RvData {
                prev: value,
                done: false,
            },
        }
    }
}

impl Rv for Value {
    fn next(&mut self, _rng: &mut Rng) -> u32 {
        self.data.done = true;

        self.data.prev
    }

    fn data(&self) -> &RvData {
        &self.data
    }
}
