use std::fmt;
use rand::Rng;
use rand::distributions::Range;
use rand::distributions::range::RangeInt;
use rand::distributions::Distribution;
use rand::sequences::Shuffle;

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

#[derive(Clone)]
pub struct Unique {
    data: ExprData,
    children: Vec<Box<Expr>>,
    visit_order: Vec<usize>,
    index: usize,
}

impl Unique {
    pub fn new(children: Vec<Box<Expr>>, rng: &mut Rng) -> Unique {
        let mut visit_order: Vec<usize> = (0..children.len()).collect();
        visit_order[..].shuffle(rng);

        Unique {
            data: ExprData {
                prev: 0,
                done: false,
            },
            children,
            visit_order,
            index: 0,
        }
    }
}

impl Expr for Unique {
    fn next(&mut self, rng: &mut Rng) -> u32 {
        let index = self.visit_order[self.index];
        self.data.prev = self.children[index].next(rng);

        self.index += 1;
        if self.index == self.children.len() {
            self.index = 0;
            self.data.done = true;
            self.visit_order[..].shuffle(rng);
        } else {
            self.data.done = false;
        }

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for Unique {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unique(")?;
        for child in self.children.iter() {
            write!(f, "{}, ", child)?;
        }
        write!(f, ")")
    }
}
