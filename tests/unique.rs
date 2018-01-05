extern crate rvs;

use std::collections::HashMap;

mod util;
use util::*;

#[test]
fn yields_each_value_once_per_cycle() {
    let a = expr_to_var("Unique(1, 2, 4, 8)").unwrap();
    let mut a = a.borrow_mut();

    let mut actual: HashMap<u32, u32> = HashMap::new();
    for i in 1..25 {
        let expected: HashMap<u32, u32> = [
            (1, i),
            (2, i),
            (4, i),
            (8, i),
        ].iter().cloned().collect();

        for _ in 0..4 {
            let entry = actual.entry(a.next()).or_insert(0);
            *entry += 1;
        }

        assert_eq!(expected, actual);
    }
}
