use std::fmt;
use rand::Rng;

use types::Expr;
use types::ExprData;
use types::Context;

#[derive(Clone)]
pub struct Sequence {
    data: ExprData,
    next: u32,
    offset: u32,
    increment: u32,
    count: u32,
    last: u32,
}

impl Sequence {
    pub fn new(args: Vec<u32>) -> Sequence {
        let (offset, increment, count) = match args.len() {
            1 => (0, 1, args[0]),
            2 => (args[0], 1, args[1]),
            3 => (args[0], args[1], args[2]),
            _ => panic!("Expected 1 to 3 arguments.  Got {}", args.len()),
        };

        // FIXME: Change to error
        if count == 0 {
            panic!("Count must be greater than 0.");
        }

        let last = offset + increment * (count - 1);

        Sequence {
            data: ExprData {
                prev: 0,
                done: false,
            },
            next: offset,
            offset,
            increment,
            count,
            last,
        }
    }
}

impl Expr for Sequence {
    fn next(&mut self, _rng: &mut Rng, _context: &Context) -> u32 {
        self.data.prev = self.next;
        self.data.done = false;

        self.next += self.increment;

        if self.next > self.last {
            self.next = self.offset;
            self.data.done = true;
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
