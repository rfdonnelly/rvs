use crate::transform::CrateRng;
use crate::model::{Expr, ExprData};

use std::fmt;

#[derive(Clone)]
pub struct Once {
    data: ExprData,
    expr: Box<Expr>,
}

impl Once {
    pub fn new(expr: Box<Expr>) -> Once {
        Once {
            data: Default::default(),
            expr,
        }
    }
}

impl Expr for Once {
    fn next(&mut self, rng: &mut CrateRng) -> u32 {
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
