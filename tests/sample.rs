extern crate rvs;

use std::collections::HashSet;

use rvs::parse;
use rvs::types::Context;

#[test]
fn basic() {
    let mut context = Context::new();
    assert!(parse("a = Sample(1, 2, 4, 8);", &mut context).is_ok());

    let a = context.get("a").unwrap();

    let expected: HashSet<u32> =
        [1, 2, 4, 8].iter().cloned().collect();
    let mut actual: HashSet<u32> = HashSet::new();

    for _ in 0..16 {
        actual.insert(a.borrow_mut().next(&context));
    }

    assert_eq!(expected, actual);
}
