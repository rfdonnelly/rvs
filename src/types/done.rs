use transform::CrateRng;
use model::{Expr, ExprData};

use std::fmt;

#[derive(Clone)]
pub struct Done {
    data: ExprData,
    expr: Box<Expr>,
}

impl Done {
    pub fn new(expr: Box<Expr>) -> Done {
        Done {
            data: ExprData {
                prev: 0,
                done: false,
            },
            expr,
        }
    }
}

impl Expr for Done {
    fn next(&mut self, rng: &mut CrateRng) -> u32 {
        self.data.prev = self.expr.next(rng);
        self.data.done = true;

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for Done {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Done({})", self.expr)
    }
}
