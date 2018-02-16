extern crate rvs;

#[test]
fn next() {
    let model = rvs::parse(&Default::default(), "a = 1; b = a;").unwrap();

    let b = model.get_variable_by_name("b").unwrap();
    let mut b = b.borrow_mut();

    assert_eq!(b.next(), 1);
}

/// Verifies that the underlying variable's state is advanced.  I.e. that variable b is not a copy
/// of variable a.
#[test]
fn next_pattern() {
    let model = rvs::parse(&Default::default(), "a = Pattern(0, 1, 2, 3); b = a;").unwrap();

    let a = model.get_variable_by_name("a").unwrap();
    let b = model.get_variable_by_name("b").unwrap();

    assert_eq!(b.borrow_mut().next(), 0);
    assert_eq!(a.borrow_mut().next(), 1);
    assert_eq!(b.borrow_mut().next(), 2);
    assert_eq!(a.borrow_mut().next(), 3);
}

#[test]
fn next_done() {
    let model = rvs::parse(&Default::default(), "a = Pattern(0, 1, 2, 3); b = a;").unwrap();

    let b = model.get_variable_by_name("b").unwrap();
    let mut b = b.borrow_mut();

    let expected: Vec<bool> = vec![false, false, false, true]
        .into_iter()
        .cycle()
        .take(32)
        .collect();
    let actual: Vec<bool> = (0..32)
        .map(|_| {
            b.next();
            b.done()
        })
        .collect();

    assert_eq!(expected, actual);
}

#[test]
fn copy() {
    let model = rvs::parse(&Default::default(), "a = 1; b = a.copy;").unwrap();

    let b = model.get_variable_by_name("b").unwrap();
    let mut b = b.borrow_mut();

    assert_eq!(b.next(), 1);
}

#[test]
fn copy_pattern() {
    let model = rvs::parse(&Default::default(), "a = Pattern(0, 1, 2, 3); b = a.copy;").unwrap();

    let a = model.get_variable_by_name("a").unwrap();
    let mut a = a.borrow_mut();
    let b = model.get_variable_by_name("b").unwrap();
    let mut b = b.borrow_mut();

    assert_eq!(b.next(), 0);
    assert_eq!(a.next(), 0);
    assert_eq!(b.next(), 1);
    assert_eq!(a.next(), 1);
    assert_eq!(b.next(), 2);
    assert_eq!(a.next(), 2);
}

#[test]
fn prev() {
    let model = rvs::parse(&Default::default(), "a = Pattern(0, 1, 2, 3); b = a.prev;").unwrap();

    let a = model.get_variable_by_name("a").unwrap();
    let b = model.get_variable_by_name("b").unwrap();

    let expected: Vec<u32> = vec![0, 1, 2, 3, 0, 1, 2, 3];

    assert_eq!(b.borrow_mut().next(), 0);
    assert_eq!(b.borrow_mut().prev(), 0);
    for (i, value) in expected.iter().enumerate() {
        let done = i % 4 == 3;
        let value = *value;
        assert_eq!(a.borrow_mut().next(), value);
        assert_eq!(a.borrow_mut().prev(), value);
        assert_eq!(a.borrow_mut().done(), done);
        assert_eq!(b.borrow_mut().next(), value);
        assert_eq!(b.borrow_mut().prev(), value);
        assert_eq!(b.borrow_mut().done(), done);
    }
}

#[test]
fn self_ref() {
    let model = rvs::parse(&Default::default(), "a = 0; a = a + 1;").unwrap();
    let a = model.get_variable_by_name("a").unwrap();
    let mut a = a.borrow_mut();

    assert_eq!(a.next(), 1);
}
