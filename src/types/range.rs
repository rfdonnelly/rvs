use rand::Rng;
use rand::distributions::Range;
use rand::distributions::range::RangeInt;
use rand::distributions::Distribution;

use types::Rv;
use types::RvData;

pub struct RangeSequence {
    data: RvData,
    range: Range<RangeInt<u32>>,
}

impl RangeSequence {
    pub fn new(l: u32, r: u32) -> RangeSequence {
        // FIXME: Range::new may panic.
        RangeSequence {
            data: RvData {
                prev: 0,
                done: false,
            },
            range: Range::new_inclusive(l, r),
        }
    }
}

impl Rv for RangeSequence {
    fn next(&mut self, rng: &mut Rng) -> u32 {
        self.data.prev = self.range.sample(rng);

        self.data.prev
    }

    fn data(&self) -> &RvData {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    mod range {
        use super::super::*;
        use types::new_rng;

        #[test]
        fn basic() {
            use std::collections::HashMap;

            let mut range = RangeSequence::new(0, 1);

            let mut rng = new_rng();
            let mut values = HashMap::new();

            for _ in 0..1000 {
                let value = range.next(&mut rng);
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

            let mut variable = RangeSequence::new(
                ::std::u32::MAX - 1,
                ::std::u32::MAX
            );

            let mut rng = new_rng();
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

            let mut variable = RangeSequence::new(
                ::std::u32::MIN,
                ::std::u32::MAX
            );

            let mut rng = new_rng();
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
