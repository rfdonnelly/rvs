use crate::transform::CrateRng;
use crate::model::{Expr, ExprData};

use std::fmt;

#[derive(Clone)]
pub struct Done {
    data: ExprData,
    expr: Box<dyn Expr>,
}

impl Done {
    pub fn new(expr: Box<dyn Expr>) -> Done {
        Done {
            data: Default::default(),
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
