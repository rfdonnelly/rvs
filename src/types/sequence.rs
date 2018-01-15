use types::Value;
use transform::CrateRng;
use model::{Expr, ExprData};

use std::num::Wrapping;
use std::fmt;

#[derive(Clone)]
pub struct Sequence {
    data: ExprData,
    next: Wrapping<u32>,
    offset: Box<Expr>,
    increment: Box<Expr>,
    count: Box<Expr>,
    remaining: u32,
}

impl Sequence {
    /// # Panics
    ///
    /// * If `args.len()` < 1 OR > 3
    /// * If count is 0
    pub fn new(mut args: Vec<Box<Expr>>, rng: &mut CrateRng) -> Sequence {
        let len = args.len();
        let mut drain = args.drain(..);
        let (offset, increment, count): (Box<Expr>, Box<Expr>, Box<Expr>) = match len {
            1 => (Box::new(Value::new(0)), Box::new(Value::new(1)), drain.next().unwrap()),
            2 => (drain.next().unwrap(), Box::new(Value::new(1)), drain.next().unwrap()),
            3 => (drain.next().unwrap(), drain.next().unwrap(), drain.next().unwrap()),
            _ => panic!("Expected 1 to 3 arguments.  Got {}", len),
        };

        let mut sequence = Sequence {
            data: ExprData {
                prev: 0,
                done: false,
            },
            next: Wrapping(0),
            offset,
            increment,
            count,
            remaining: 0,
        };

        sequence.init_params(rng);

        sequence
    }

    fn init_params(&mut self, rng: &mut CrateRng) {
        self.init_next(rng);
        self.init_increment(rng);
        self.init_remaining(rng);
    }

    fn init_increment(&mut self, rng: &mut CrateRng) {
        self.increment.next(rng);
    }

    fn init_remaining(&mut self, rng: &mut CrateRng) {
        self.remaining = self.count.next(rng);

        if self.remaining == 0 {
            panic!("count == 0, count must be nonzero for {}", self);
        }
    }

    fn init_next(&mut self, rng: &mut CrateRng) {
        self.next = Wrapping(self.offset.next(rng));
    }
}

impl Expr for Sequence {
    fn next(&mut self, rng: &mut CrateRng) -> u32 {
        self.data.prev = self.next.0;
        self.data.done = false;

        self.next += Wrapping(self.increment.prev());
        self.remaining -= 1;

        if self.remaining == 0 {
            self.data.done = true;
            self.init_params(rng);
        }

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sequence({}, {}, {})", self.offset, self.increment, self.count)
    }
}
