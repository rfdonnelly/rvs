extern crate rvs;

use std::collections::HashSet;

use rvs::Context;

#[test]
fn basic() {
    let mut context = Context::new();
    rvs::parse("a = Sample(1, 2, 4, 8);", &mut context).unwrap();
    rvs::transform(&mut context).unwrap();

    let a = context.get("a").unwrap();

    let expected: HashSet<u32> =
        [1, 2, 4, 8].iter().cloned().collect();
    let mut actual: HashSet<u32> = HashSet::new();

    for _ in 0..16 {
        actual.insert(a.borrow_mut().next());
    }

    assert_eq!(expected, actual);
}
