use std::fmt;
use std::u32;
use rand::Rng;
use rand::distributions::Range as RandRange;
use rand::distributions::Sample;
use rand::distributions::IndependentSample;

use transform::CrateRng;
use model::{Expr, ExprData};

#[derive(Clone)]
pub struct Range {
    data: ExprData,
    l: u32,
    r: u32,
    range: RandRangeInclusive,
}

#[derive(Clone)]
pub struct RandRangeInclusive {
    range: RandRange<u32>,
    use_range: bool,
    offset: bool,
}

impl RandRangeInclusive {
    fn new(low: u32, high: u32) -> RandRangeInclusive {
        // Implement the inclusive range [x, y] using the exlusive range [x, y + 1) by handling
        // three different cases:
        //
        // * The range [u32::MIN, u32::MAX]
        //
        //   Cannot use rand::distributions::Range.  Use RNG directly.
        //
        //   [x, y] => [x, y]
        //
        // * The range [x, u32::MAX]
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
            (u32::MIN, u32::MAX) => (u32::MIN, u32::MAX, false, false),
            // Sample with Range + offset
            (x, u32::MAX) => (x - 1, u32::MAX, true, true),
            // Sample with Range normally
            (x, y) => (x, y + 1, true, false),
        };

        RandRangeInclusive {
            offset,
            use_range,
            range: RandRange::new(x, y),
        }
    }
}

impl IndependentSample<u32> for RandRangeInclusive {
    fn ind_sample<R: Rng>(&self, rng: &mut R) -> u32 {
        // Should never see this case.  Could cause a panic due to overflow.
        debug_assert!(self.use_range || !self.offset);

        let sample = if self.use_range {
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

impl Sample<u32> for RandRangeInclusive {
    fn sample<R: Rng>(&mut self, rng: &mut R) -> u32 {
        self.ind_sample(rng)
    }
}

impl Range {
    pub fn new(l: u32, r: u32) -> Range {
        let limits = if r > l { (l, r) } else { (r, l) };

        Range {
            data: ExprData {
                prev: 0,
                done: false,
            },
            l,
            r,
            range: RandRangeInclusive::new(limits.0, limits.1),
        }
    }
}

impl Expr for Range {
    fn next(&mut self, rng: &mut CrateRng) -> u32 {
        self.data.prev = self.range.sample(rng);
        self.data.done = true;

        self.data.prev
    }

    fn data(&self) -> &ExprData {
        &self.data
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[0x{:x}, 0x{:x}]", self.l, self.r)
    }
}

#[cfg(test)]
mod tests {
    mod range {
        use super::super::*;
        use transform::Seed;

        #[test]
        fn basic() {
            use std::collections::HashMap;

            let mut range = Range::new(0, 1);

            let mut rng = Seed::from_u32(0).to_rng();
            let mut values = HashMap::new();

            for _ in 0..1000 {
                let value = range.next(&mut rng);
                let entry = values.entry(value).or_insert(0);
                *entry += 1;
                assert!(value == 0 || value == 1);
            }

            let num_zeros = values[&0];
            let num_ones = values[&1];

            println!("num_zeros:{} num_ones:{}", num_zeros, num_ones);
            assert!(num_zeros > 450 && num_zeros < 550);
            assert!(num_ones > 450 && num_ones < 550);
        }

        #[test]
        fn max_max() {
            use std::collections::HashMap;

            let mut variable = Range::new(u32::MAX - 1, u32::MAX);

            let mut rng = Seed::from_u32(0).to_rng();
            let mut values = HashMap::new();

            for _ in 0..100 {
                let value = variable.next(&mut rng);
                let entry = values.entry(value).or_insert(0);
                *entry += 1;
                assert!(value == u32::MAX - 1 || value == u32::MAX);
            }

            assert!(values[&(u32::MAX - 1)] > 0);
            assert!(values[&u32::MAX] > 0);
        }

        #[test]
        #[ignore]
        fn full_range() {
            use std::collections::HashMap;

            let mut variable = Range::new(u32::MIN, u32::MAX);

            let mut rng = Seed::from_u32(0).to_rng();
            let mut values = HashMap::new();

            for _ in 0u64..0x2_0000_0000u64 {
                let value = variable.next(&mut rng);
                if value == u32::MIN || value == u32::MAX {
                    let entry = values.entry(value).or_insert(0);
                    *entry += 1;
                }
            }

            assert!(values[&u32::MIN] > 0);
            assert!(values[&u32::MAX] > 0);
        }
    }
}
