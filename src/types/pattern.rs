use std::fmt;
use rand::Rng;

use types::Rv;
use types::RvData;

pub struct Pattern {
    data: RvData,
    index: usize,
    children: Vec<Box<Rv>>,
}

impl Pattern {
    pub fn new(children: Vec<Box<Rv>>) -> Pattern {
        Pattern {
            data: RvData {
                prev: 0,
                done: false,
            },
            index: 0,
            children,
        }
    }
}

impl Rv for Pattern {
    fn next(&mut self, rng: &mut Rng) -> u32 {
        self.data.prev = self.children[self.index].next(rng);
        self.index = (self.index + 1) % self.children.len();
        self.data.prev
    }

    fn data(&self) -> &RvData {
        &self.data
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pattern(")?;
        for child in self.children.iter() {
            write!(f, "{}, ", child)?;
        }
        write!(f, ")")
    }
}
