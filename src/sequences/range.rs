use rand::Rng;
use rand::SeedableRng;
use rand::chacha::ChaChaRng;
use rand::distributions::Range;
use rand::distributions::Sample;
use rand::distributions::IndependentSample;

use super::Sequence;
use super::Value;

pub struct RangeSequence {
    prev: u32,
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
        let (x, y, use_range, offset) = match (low, high) {
            // Sample directly from RNG w/o Range
            (::std::u32::MIN, ::std::u32::MAX) => (::std::u32::MIN, ::std::u32::MAX, false, false),
            // Sample with Range + offset
            (x, ::std::u32::MAX) => (x - 1, ::std::u32::MAX, true, true),
            // Sample with Range normally
            (x, y) => (x, y + 1, true, false)
        };

        RangeInclusive {
            offset: offset,
            use_range: use_range,
            range: Range::new(x, y),
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

impl RangeSequence {
    pub fn new(l: &mut Sequence, r: &mut Sequence) -> RangeSequence {
        // FIXME: Range::new may panic.
        // FIXME: Allow non-const seed
        RangeSequence {
            prev: 0,
            rng: ChaChaRng::from_seed(&[0x0000_0000]),
            range: RangeInclusive::new(l.next(), r.next()),
        }
    }
}

impl Sequence for RangeSequence {
    fn next(&mut self) -> u32 {
        self.prev = self.range.ind_sample(&mut self.rng);

        self.prev
    }

    fn prev(&self) -> u32 {
        self.prev
    }

    fn done(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    mod range {
        use super::super::*;

        #[test]
        fn basic() {
            use std::collections::HashMap;

            let mut range = RangeSequence::new(
                &mut Value::new(0),
                &mut Value::new(1)
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

            let mut sequence = RangeSequence::new(
                &mut Value::new(::std::u32::MAX - 1),
                &mut Value::new(::std::u32::MAX)
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

            let mut sequence = RangeSequence::new(
                &mut Value::new(::std::u32::MIN),
                &mut Value::new(::std::u32::MAX)
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
}
