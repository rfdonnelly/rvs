extern crate rvs;

use std::collections::HashSet;

#[test]
fn basic() {
    let model = rvs::parse(
        Default::default(),
        "a = Sample(1, 2, 4, 8);"
        ).unwrap();

    let a = model.get_variable_by_name("a").unwrap();

    let expected: HashSet<u32> =
        [1, 2, 4, 8].iter().cloned().collect();
    let mut actual: HashSet<u32> = HashSet::new();

    for _ in 0..16 {
        actual.insert(a.borrow_mut().next());
    }

    assert_eq!(expected, actual);
}
