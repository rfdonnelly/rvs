use std::fmt;
use rand::Rng;

use types::RvWeak;
use types::Expr;
use types::ExprData;
use types::Context;

#[derive(Clone)]
pub struct Next {
    variable: RvWeak,
    variable_name: String,
    data: ExprData,
}

#[derive(Clone)]
pub struct Prev {
    variable: RvWeak,
    variable_name: String,
    data: ExprData,
}

impl Next {
    pub fn new(variable_name: &str, variable: RvWeak) -> Next {
        Next {
            variable,
            variable_name: variable_name.into(),
            data: ExprData {
                prev: 0,
                done: false,
            },
        }
    }
}

impl Expr for Next {
    /// # Panics
    ///
    /// * If variable no longer exists.
    fn next(&mut self, _rng: &mut Rng, context: &Context) -> u32 {
        let variable = self.variable.upgrade().unwrap();
        self.data.prev = variable.borrow_mut().next(context);
        self.data.done = variable.borrow().done();

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for Next {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.variable_name)
    }
}

impl Prev {
    pub fn new(variable_name: &str, variable: RvWeak) -> Prev {
        Prev {
            variable,
            variable_name: variable_name.into(),
            data: ExprData {
                prev: 0,
                done: false,
            },
        }
    }
}

impl Expr for Prev {
    /// # Panics
    ///
    /// * If variable no longer exists.
    fn next(&mut self, _rng: &mut Rng, context: &Context) -> u32 {
        let variable = self.variable.upgrade().unwrap();
        self.data.prev = variable.borrow().prev();
        self.data.done = variable.borrow().done();

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for Prev {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.prev", self.variable_name)
    }
}
