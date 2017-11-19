use std::fmt;
use rand::Rng;
use rand::distributions::Range;
use rand::distributions::range::RangeInt;
use rand::distributions::Distribution;

use types::Rv;
use types::RvData;

pub struct Sample {
    data: RvData,
    children: Vec<Box<Rv>>,
    range: Range<RangeInt<usize>>,
}

impl Sample {
    pub fn new(children: Vec<Box<Rv>>) -> Sample {
        Sample {
            data: RvData {
                prev: 0,
                done: false,
            },
            range: Range::new(0, children.len()),
            children,
        }
    }
}

impl Rv for Sample {
    fn next(&mut self, rng: &mut Rng) -> u32 {
        let idx = self.range.sample(rng);

        self.data.prev = self.children[idx].next(rng);

        self.data.prev
    }

    fn data(&self) -> &RvData {
        &self.data
    }
}

impl fmt::Display for Sample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sample(")?;
        for child in self.children.iter() {
            write!(f, "{}, ", child)?;
        }
        write!(f, ")")
    }
}
