extern crate rvs;

use std::collections::HashMap;

use rvs::parse;
use rvs::types::Context;

#[test]
fn basic() {
    let mut context = Context::new();
    assert!(parse("a = { 10: 0, 90: 1 };", &mut context).is_ok());

    let a = context.get("a").unwrap();

    let mut results: HashMap<u32, u32> = HashMap::new();

    for _ in 0..1000 {
        let result = a.borrow_mut().next();
        let entry = results.entry(result).or_insert(0);
        *entry += 1;
    }

    assert!(results[&0] >= 100 - 10 && results[&0] <= 100 + 10);
    assert!(results[&1] >= 900 - 10 && results[&1] <= 900 + 10);
}
