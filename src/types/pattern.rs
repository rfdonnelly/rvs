use std::fmt;
use rand::Rng;

use types::Expr;
use types::ExprData;
use types::Context;

pub struct Pattern {
    data: ExprData,
    index: usize,
    children: Vec<Box<Expr>>,
}

impl Pattern {
    pub fn new(children: Vec<Box<Expr>>) -> Pattern {
        Pattern {
            data: ExprData {
                prev: 0,
                done: false,
            },
            index: 0,
            children,
        }
    }
}

impl Expr for Pattern {
    fn next(&mut self, rng: &mut Rng, context: &Context) -> u32 {
        self.data.prev = self.children[self.index].next(rng, context);
        self.index = (self.index + 1) % self.children.len();
        self.data.prev
    }

    fn data(&self) -> &ExprData {
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
