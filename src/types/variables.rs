use std::fmt;

use crate::transform::CrateRng;
use crate::model::{Expr, ExprData, VariableWeak};

#[derive(Clone)]
pub struct Next {
    variable: VariableWeak,
    variable_name: String,
    data: ExprData,
}

#[derive(Clone)]
pub struct Prev {
    variable: VariableWeak,
    variable_name: String,
    data: ExprData,
}

impl Next {
    pub fn new(variable_name: &str, variable: VariableWeak) -> Next {
        Next {
            variable,
            variable_name: variable_name.into(),
            data: Default::default(),
        }
    }
}

impl Expr for Next {
    /// # Errors
    ///
    /// If Weak pointer cannot be upgraded, next() will return previous value.
    fn next(&mut self, _rng: &mut CrateRng) -> u32 {
        if let Some(variable) = self.variable.upgrade() {
            self.data.prev = variable.borrow_mut().next();
            self.data.done = variable.borrow().done();
        }

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
    pub fn new(variable_name: &str, variable: VariableWeak) -> Prev {
        Prev {
            variable,
            variable_name: variable_name.into(),
            data: Default::default(),
        }
    }
}

impl Expr for Prev {
    /// # Errors
    ///
    /// If Weak pointer cannot be upgraded, next() will return previous value.
    fn next(&mut self, _rng: &mut CrateRng) -> u32 {
        if let Some(variable) = self.variable.upgrade() {
            self.data.prev = variable.borrow().prev();
            self.data.done = variable.borrow().done();
        }

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
