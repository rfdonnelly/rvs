extern crate rvs;

mod util;
use crate::util::*;

use std::collections::HashMap;

#[test]
fn distribution() {
    let a = expr_to_var("{1: 0, 9: 1}").unwrap();
    let mut a = a.borrow_mut();

    let expected: HashMap<u32, u32> = [(0, 1), (1, 9)].iter().cloned().collect();

    for _ in 0..100 {
        let mut actual: HashMap<u32, u32> = HashMap::new();
        for _ in 0..10 {
            let entry = actual.entry(a.next()).or_insert(0);
            *entry += 1;
        }

        assert_eq!(expected, actual);
    }
}
