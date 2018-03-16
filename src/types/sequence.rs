use types::Value;
use transform::CrateRng;
use model::{Expr, ExprData};

use std::num::Wrapping;
use std::fmt;

#[derive(Clone)]
pub struct Sequence {
    data: ExprData,
    next: Wrapping<u32>,
    first: Box<Expr>,
    last: Box<Expr>,
    increment: Box<Expr>,
    compare: bool,
}

impl Sequence {
    /// # Panics
    ///
    /// * If `args.len()` < 1 OR > 3
    /// * If increment is 0
    pub fn new(mut args: Vec<Box<Expr>>, rng: &mut CrateRng) -> Sequence {
        let len = args.len();
        let mut drain = args.drain(..);
        let (first, last, increment): (Box<Expr>, Box<Expr>, Box<Expr>) = match len {
            1 => (
                Box::new(Value::new(0)),
                drain.next().unwrap(),
                Box::new(Value::new(1)),
            ),
            2 => (
                drain.next().unwrap(),
                drain.next().unwrap(),
                Box::new(Value::new(1)),
            ),
            3 => (
                drain.next().unwrap(),
                drain.next().unwrap(),
                drain.next().unwrap(),
            ),
            _ => panic!("Expected 1 to 3 arguments.  Got {}", len),
        };

        let mut sequence = Sequence {
            data: Default::default(),
            next: Wrapping(0),
            first,
            last,
            increment,
            compare: false,
        };

        sequence.init_params(rng);

        sequence
    }

    fn init_params(&mut self, rng: &mut CrateRng) {
        self.init_next(rng);
        self.init_last(rng);
        self.init_increment(rng);

        self.compare = self.compare();
    }

    fn init_increment(&mut self, rng: &mut CrateRng) {
        let increment = self.increment.next(rng);

        if increment == 0 {
            panic!(
                "the increment sub-expression `{}` returned 0 in the expression `{}`",
                self.increment, self
            );
        }
    }

    fn init_last(&mut self, rng: &mut CrateRng) {
        self.last.next(rng);
    }

    fn init_next(&mut self, rng: &mut CrateRng) {
        self.next = Wrapping(self.first.next(rng));
    }

    fn compare(&self) -> bool {
        if self.next.0 == self.last.prev() {
            self.compare
        } else {
            self.next.0 < self.last.prev()
        }
    }

    fn is_last(&self) -> bool {
        self.next.0 == self.last.prev()
    }

    fn past_last(&self) -> bool {
        self.compare != self.compare()
    }

    fn done(&mut self, rng: &mut CrateRng) {
        self.data.done = true;
        self.init_params(rng);
    }
}

impl Expr for Sequence {
    /// # Panics
    ///
    /// * If increment returns 0
    fn next(&mut self, rng: &mut CrateRng) -> u32 {
        self.data.prev = self.next.0;
        self.data.done = false;

        if self.is_last() {
            self.done(rng);
        } else {
            self.next += Wrapping(self.increment.prev());

            if self.past_last() {
                self.done(rng);
            }
        }

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Sequence({}, {}, {})",
            self.first, self.last, self.increment
        )
    }
}
