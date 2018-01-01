use std::fmt;
use rand::Rng;
use rand::distributions::Range as RndRange;
use rand::distributions::range::RangeInt;
use rand::distributions::Distribution;

use model::{Expr, ExprData};

#[derive(Clone)]
pub struct Range {
    data: ExprData,
    l: u32,
    r: u32,
    range: RndRange<RangeInt<u32>>,
}

impl Range {
    pub fn new(l: u32, r: u32) -> Range {
        let limits = if r > l {
            (l, r)
        } else {
            (r, l)
        };

        Range {
            data: ExprData {
                prev: 0,
                done: false,
            },
            l: l,
            r: r,
            range: RndRange::new_inclusive(limits.0, limits.1),
        }
    }
}

impl Expr for Range {
    fn next(&mut self, rng: &mut Rng) -> u32 {
        self.data.prev = self.range.sample(rng);

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
            assert!(num_zeros > 487 && num_zeros < 513);
            assert!(num_ones > 487 && num_ones < 513);
        }

        #[test]
        fn max_max() {
            use std::collections::HashMap;

            let mut variable = Range::new(
                ::std::u32::MAX - 1,
                ::std::u32::MAX
            );

            let mut rng = Seed::from_u32(0).to_rng();
            let mut values = HashMap::new();

            for _ in 0..100 {
                let value = variable.next(&mut rng);
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

            let mut variable = Range::new(
                ::std::u32::MIN,
                ::std::u32::MAX
            );

            let mut rng = Seed::from_u32(0).to_rng();
            let mut values = HashMap::new();

            for _ in 0u64..0x2_0000_0000u64 {
                let value = variable.next(&mut rng);
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
