use std::fmt;
use rand::Rng;

use model::{Expr, ExprData};

#[derive(Clone)]
pub struct Value {
    data: ExprData,
}

impl Value {
    pub fn new(value: u32) -> Value {
        Value {
            data: ExprData {
                prev: value,
                done: false,
            },
        }
    }
}

impl Expr for Value {
    fn next(&mut self, _rng: &mut Rng) -> u32 {
        self.data.done = true;

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x}", self.data.prev)
    }
}
