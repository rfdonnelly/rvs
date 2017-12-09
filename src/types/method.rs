use std::fmt;
use rand::Rng;

use types::Expr;
use types::ExprData;
use types::Context;

pub struct Next {
    variable_name: String,
    variable_index_valid: bool,
    variable_index: usize,
    data: ExprData,
}

impl Next {
    pub fn new(variable_name: &str) -> Next {
        Next {
            variable_name: variable_name.into(),
            variable_index_valid: false,
            variable_index: 0,
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
    /// * If variable not found in context
    fn next(&mut self, _rng: &mut Rng, context: &Context) -> u32 {
        if !self.variable_index_valid {
            self.variable_index = context.variables.get_index(&self.variable_name).unwrap();
        }

        let variable = context.variables.get_by_index(self.variable_index).unwrap();

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
