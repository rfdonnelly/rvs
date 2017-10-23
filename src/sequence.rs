use ast::Opcode;

use rand::SeedableRng;
use rand::chacha::ChaChaRng;
use rand::distributions::Range;
use rand::distributions::IndependentSample;

pub trait Sequence {
    fn next(&mut self) -> u32;
    fn last(&self) -> u32;
}

pub struct Value {
    last: u32,
}

pub struct Expr<'a> {
    last: u32,
    operation: Opcode,
    l: &'a mut Sequence,
    r: &'a mut Sequence,
}

pub struct RangeSequence {
    last: u32,
    rng: ChaChaRng,
    range: Range<u32>,
}

impl Value {
    pub fn new(value: u32) -> Value {
        Value {
            last: value,
        }
    }
}

impl Sequence for Value {
    fn next(&mut self) -> u32 {
        self.last
    }

    fn last(&self) -> u32 {
        self.last
    }
}

impl<'a> Expr<'a> {
    pub fn new(l: &'a mut Sequence, operation: Opcode, r: &'a mut Sequence) -> Expr<'a> {
        Expr {
            last: 0,
            operation: operation,
            l: l,
            r: r,
        }
    }
}

impl<'a> Sequence for Expr<'a> {
    fn next(&mut self) -> u32 {
        self.last = match self.operation {
            Opcode::Add => self.l.next() + self.r.next(),
            Opcode::Subtract => self.l.next() - self.r.next(),
            Opcode::Multiply => self.l.next() * self.r.next(),
            Opcode::Divide => self.l.next() / self.r.next(),
        };

        self.last
    }

    fn last(&self) -> u32 {
        self.last
    }
}

impl<'a> RangeSequence {
    pub fn new(l: &'a mut Sequence, r: &'a mut Sequence) -> RangeSequence {
        // FIXME: Range::new may panic.
        // FIXME: Allow non-const seed
        // FIXME?: Range is from x (inclusive) to y (exclusive) so add 1 to y to make it inclusive.
        // Does this prevent max int (0xffffffff) from being covered?
        RangeSequence {
            last: 0,
            rng: ChaChaRng::from_seed(&[0x0000_0000]),
            range: Range::new(l.next(), r.next() + 1),
        }
    }
}

impl<'a> Sequence for RangeSequence {
    fn next(&mut self) -> u32 {
        self.last = self.range.ind_sample(&mut self.rng);

        self.last
    }

    fn last(&self) -> u32 {
        self.last
    }
}

mod tests {
    use super::*;

    #[test]
    fn expr() {
        let mut v0 = Value::new(1);
        let mut v1 = Value::new(2);

        let mut expr = Expr::new(
            &mut v0,
            Opcode::Add,
            &mut v1,
        );

        assert_eq!(expr.next(), 3);
        assert_eq!(expr.next(), 3);
    }

    #[test]
    fn range() {
        use std::collections::HashMap;

        let mut v0 = Value::new(0);
        let mut v1 = Value::new(1);

        let mut range = RangeSequence::new(
            &mut v0,
            &mut v1
        );

        let mut values = HashMap::new();

        for _ in 0..1000 {
            let value = range.next();
            let entry = values.entry(value).or_insert(0);
            *entry += 1;
            assert!(value == 0 || value == 1);
        }

        let num_zeros = values[&0];
        let num_ones = values[&1];

        assert!(num_zeros > 490 && num_zeros < 510);
        assert!(num_ones > 490 && num_ones < 510);
    }
}
