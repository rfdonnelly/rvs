extern crate rvs;

mod util;
use util::*;

use std::collections::HashMap;

#[test]
fn distribution() {
    let a = expr_to_var("{10: 0, 90: 1}").unwrap();
    let mut a = a.borrow_mut();

    let mut results: HashMap<u32, u32> = HashMap::new();

    for _ in 0..1000 {
        let entry = results.entry(a.next()).or_insert(0);
        *entry += 1;
    }

    assert!(results[&0] >= 100 - 10 && results[&0] <= 100 + 10);
    assert!(results[&1] >= 900 - 10 && results[&1] <= 900 + 10);
}
