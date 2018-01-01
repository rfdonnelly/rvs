use std::fmt;
use rand::Rng;
use rand::distributions::Range;
use rand::distributions::range::RangeInt;
use rand::distributions::Distribution;

use model::{Expr, ExprData};

#[derive(Clone)]
pub struct Sample {
    data: ExprData,
    children: Vec<Box<Expr>>,
    range: Range<RangeInt<usize>>,
}

impl Sample {
    pub fn new(children: Vec<Box<Expr>>) -> Sample {
        Sample {
            data: ExprData {
                prev: 0,
                done: false,
            },
            range: Range::new(0, children.len()),
            children,
        }
    }
}

impl Expr for Sample {
    fn next(&mut self, rng: &mut Rng) -> u32 {
        let idx = self.range.sample(rng);

        self.data.prev = self.children[idx].next(rng);

        self.data.prev
    }

    fn data(&self) -> &ExprData {
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
