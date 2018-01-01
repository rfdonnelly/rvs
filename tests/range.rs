extern crate rvs;

use std::collections::HashSet;

#[test]
fn reverse() {
    let model = rvs::parse(
        Default::default(),
        "a = [1, 0];"
        ).unwrap();

    let a = model.get_variable_by_name("a").unwrap();

    let expected: HashSet<u32> =
        [0, 1].iter().cloned().collect();
    let mut actual: HashSet<u32> = HashSet::new();

    for _ in 0..10 {
        actual.insert(a.borrow_mut().next());
    }

    assert_eq!(expected, actual);
}

#[test]
fn same() {
    let model = rvs::parse(
        Default::default(),
        "a = [1, 1];"
        ).unwrap();

    let a = model.get_variable_by_name("a").unwrap();

    for _ in 0..10 {
        assert_eq!(a.borrow_mut().next(), 1);
    }
}
