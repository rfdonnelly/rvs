use std::fmt;

use crate::transform::CrateRng;
use crate::model::{Expr, ExprData};

#[derive(Clone)]
pub struct Pattern {
    data: ExprData,
    children: Vec<Box<dyn Expr>>,
    current_child: usize,
}

impl Pattern {
    pub fn new(children: Vec<Box<dyn Expr>>) -> Pattern {
        Pattern {
            data: Default::default(),
            children,
            current_child: 0,
        }
    }
}

impl Expr for Pattern {
    fn next(&mut self, rng: &mut CrateRng) -> u32 {
        self.data.prev = self.children[self.current_child].next(rng);

        if self.children[self.current_child].done() {
            self.current_child = (self.current_child + 1) % self.children.len();
            self.data.done = self.current_child == 0;
        } else {
            self.data.done = false;
        }

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pattern(")?;
        for child in &self.children {
            write!(f, "{}, ", child)?;
        }
        write!(f, ")")
    }
}
