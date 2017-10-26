use ast::Opcode;
use ast::Node;

use rand::Rng;
use rand::SeedableRng;
use rand::chacha::ChaChaRng;
use rand::distributions::Range;
use rand::distributions::Sample;
use rand::distributions::IndependentSample;

pub trait Sequence {
    fn next(&mut self) -> u32;
    fn last(&self) -> u32;
}

pub fn sequence_from_ast(node: &Node) -> Box<Sequence> {
    match *node {
        Node::Identifier(_) => panic!("Not supported"),
        Node::Assignment(_, _) => panic!("Not supported"),
        Node::Range(ref bx, ref by) => {
            Box::new(
                RangeSequence::new(
                    &mut sequence_from_ast(bx),
                    &mut sequence_from_ast(by)
                )
            )
        }
        Node::Number(x) => Box::new(Value::new(x)),
        Node::Operation(ref bx, ref op, ref by) => {
            Box::new(
                Expr::new(
                    sequence_from_ast(bx),
                    op.clone(),
                    sequence_from_ast(by)
                )
            )
        }
    }
}

pub struct Value {
    last: u32,
}

pub struct Expr {
    last: u32,
    operation: Opcode,
    l: Box<Sequence>,
    r: Box<Sequence>,
}

pub struct RangeSequence {
    last: u32,
    rng: ChaChaRng,
    range: RangeInclusive
}

pub struct RangeInclusive {
    range: Range<u32>,
    use_range: bool,
    offset: bool,
}

impl RangeInclusive {
    fn new(low: u32, high: u32) -> RangeInclusive {
        let mut low = low;
        let mut high = high;
        let mut use_range = true;
        let mut offset = false;

        // Implement the inclusive range [x, y] using the exlusive range [x, y + 1) by handling
        // three different cases:
        //
        // * The range [::std::u32::MIN, ::std::u32::MAX]
        //
        //   Cannot use rand::distributions::Range.  Use RNG directly.
        //
        //   [x, y] => [x, y]
        //
        // * The range [x, ::std::u32::MAX]
        //
        //   Can use rand::distributions::Range but must adjust the range down artifically, then
        //   re-adjust up after sampling.
        //
        //   [x, y] => [x - 1, y) + 1
        //
        // * All other ranges
        //
        //   Use rand::distributions::Range normally.
        //
        //   [x, y] => [x, y + 1)
        if high == ::std::u32::MAX {
            if low == ::std::u32::MIN {
                // Sample directly from RNG w/o Range
                use_range = false;
                high -= 1; // Prevent panic on Range::new
            } else {
                // Sample with Range + offset
                offset = true;
                low -= 1;
                high -= 1;
            }
        }

        RangeInclusive {
            offset: offset,
            use_range: use_range,
            range: Range::new(low, high + 1),
        }
    }
}

impl IndependentSample<u32> for RangeInclusive {
    fn ind_sample<R: Rng>(&self, rng: &mut R) -> u32 {
        // Should never see this case.  Could cause a panic due to overflow.
        assert!(!(self.use_range == false && self.offset == true));

        let sample =
            if self.use_range {
                self.range.ind_sample(rng)
            } else {
                rng.gen()
            };

        if self.offset {
            sample + 1
        } else {
            sample
        }
    }
}

impl Sample<u32> for RangeInclusive {
    fn sample<R: Rng>(&mut self, rng: &mut R) -> u32 {
        self.ind_sample(rng)
    }
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

impl<'a> Expr {
    pub fn new(l: Box<Sequence>, operation: Opcode, r: Box<Sequence>) -> Expr {
        Expr {
            last: 0,
            operation: operation,
            l: l,
            r: r,
        }
    }
}

impl<'a> Sequence for Expr {
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
    pub fn new(l: &mut Box<Sequence>, r: &mut Box<Sequence>) -> RangeSequence {
        // FIXME: Range::new may panic.
        // FIXME: Allow non-const seed
        RangeSequence {
            last: 0,
            rng: ChaChaRng::from_seed(&[0x0000_0000]),
            range: RangeInclusive::new(l.next(), r.next()),
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
        let v0 = Box::new(Value::new(1));
        let v1 = Box::new(Value::new(2));

        let mut expr = Expr::new(
            v0,
            Opcode::Add,
            v1,
        );

        assert_eq!(expr.next(), 3);
        assert_eq!(expr.next(), 3);
    }

    mod range {
        use super::super::*;

        #[test]
        fn basic() {
            use std::collections::HashMap;

            let mut v0: Box<Sequence> = Box::new(Value::new(0));
            let mut v1: Box<Sequence> = Box::new(Value::new(1));

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

        #[test]
        fn max_max() {
            use std::collections::HashMap;

            let mut v0: Box<Sequence> = Box::new(Value::new(::std::u32::MAX - 1));
            let mut v1: Box<Sequence> = Box::new(Value::new(::std::u32::MAX));
            let mut sequence = RangeSequence::new(
                &mut v0,
                &mut v1
            );

            let mut values = HashMap::new();
            for _ in 0..100 {
                let value = sequence.next();
                let entry = values.entry(value).or_insert(0);
                *entry += 1;
                assert!(value == ::std::u32::MAX - 1 || value == ::std::u32::MAX);
            }

            assert!(values[&(::std::u32::MAX - 1)] > 0);
            assert!(values[&::std::u32::MAX] > 0);
        }

        #[test]
        #[ignore]
        fn full_range() {
            use std::collections::HashMap;

            let mut v0: Box<Sequence> = Box::new(Value::new(::std::u32::MIN));
            let mut v1: Box<Sequence> = Box::new(Value::new(::std::u32::MAX));
            let mut sequence = RangeSequence::new(
                &mut v0,
                &mut v1
            );

            let mut values = HashMap::new();
            for _ in 0u64..0x2_0000_0000u64 {
                let value = sequence.next();
                if value == ::std::u32::MIN || value == ::std::u32::MAX {
                    let entry = values.entry(value).or_insert(0);
                    *entry += 1;
                }
            }

            assert!(values[&::std::u32::MIN] > 0);
            assert!(values[&::std::u32::MAX] > 0);
        }
    }

    mod sequence_from_ast {
        use super::super::*;

        #[test]
        fn number() {
            let ast = Node::Number(4);
            let mut sequence = sequence_from_ast(&ast);

            assert_eq!(sequence.next(), 4);
        }

        #[test]
        fn range() {
            use std::collections::HashMap;

            let ast = Node::Range(
                Box::new(Node::Number(3)),
                Box::new(Node::Number(4))
            );
            let mut sequence = sequence_from_ast(&ast);

            let mut values = HashMap::new();

            for _ in 0..10 {
                let value = sequence.next();
                let entry = values.entry(value).or_insert(0);
                *entry += 1;
                assert!(value == 3 || value == 4);
            }

            assert!(values[&3] > 0);
            assert!(values[&4] > 0);
        }
    }
}
