use model::{Expr, ExprData};

use rand::Rng;
use std::fmt;

#[derive(Clone)]
pub struct Once {
    data: ExprData,
    expr: Box<Expr>,
}

impl Once {
    pub fn new(expr: Box<Expr>) -> Once {
        Once {
            data: ExprData {
                prev: 0,
                done: false,
            },
            expr,
        }
    }
}

impl Expr for Once {
    fn next(&mut self, rng: &mut Rng) -> u32 {
        if !self.data.done {
            self.data.prev = self.expr.next(rng);
        }
        self.data.done = true;

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for Once {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Once({})", self.expr)
    }
}
