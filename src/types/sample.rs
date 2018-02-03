use std::fmt;
use rand::Rng;
use rand::distributions::{Range, Sample as RandSample};

use transform::CrateRng;
use model::{Expr, ExprData};

#[derive(Clone)]
pub struct Sample {
    data: ExprData,
    children: Vec<Box<Expr>>,
    current_child: Option<usize>,
    range: Range<usize>,
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
            current_child: None,
        }
    }
}

impl Expr for Sample {
    fn next(&mut self, rng: &mut CrateRng) -> u32 {
        let index = match self.current_child {
            Some(index) => index,
            None => self.range.sample(rng),
        };

        self.data.prev = self.children[index].next(rng);
        self.data.done = self.children[index].done();
        self.current_child = if self.data.done { None } else { Some(index) };

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for Sample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sample(")?;
        for child in &self.children {
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
    current_child: usize,
}

impl Unique {
    pub fn new(children: Vec<Box<Expr>>, rng: &mut CrateRng) -> Unique {
        let mut visit_order: Vec<usize> = (0..children.len()).collect();
        rng.shuffle(&mut visit_order);

        Unique {
            data: ExprData {
                prev: 0,
                done: false,
            },
            children,
            visit_order,
            current_child: 0,
        }
    }
}

impl Expr for Unique {
    fn next(&mut self, rng: &mut CrateRng) -> u32 {
        let index = self.visit_order[self.current_child];
        self.data.prev = self.children[index].next(rng);

        if self.children[index].done() {
            self.current_child += 1;
            if self.current_child == self.children.len() {
                self.current_child = 0;
                self.data.done = true;
                rng.shuffle(&mut self.visit_order);
            }
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
        for child in &self.children {
            write!(f, "{}, ", child)?;
        }
        write!(f, ")")
    }
}
