extern crate rvs;

#[test]
fn next() {
    let model = rvs::parse(
        Default::default(),
    "a = 1; b = a;"
        ).unwrap();

    let b = model.get_variable_by_name("b").unwrap();

    assert_eq!(b.borrow_mut().next(), 1);
}

/// Verifies that the underlying variable's state is advanced
#[test]
fn next_pattern() {
    let model = rvs::parse(
        Default::default(),
    "a = Pattern(0, 1, 2, 3); b = a;"
        ).unwrap();

    let a = model.get_variable_by_name("a").unwrap();
    let b = model.get_variable_by_name("b").unwrap();

    assert_eq!(b.borrow_mut().next(), 0);
    assert_eq!(a.borrow_mut().next(), 1);
    assert_eq!(b.borrow_mut().next(), 2);
    assert_eq!(a.borrow_mut().next(), 3);
}

#[test]
fn copy() {
    let model = rvs::parse(
        Default::default(),
    "a = 1; b = a.copy;"
        ).unwrap();

    let b = model.get_variable_by_name("b").unwrap();

    assert_eq!(b.borrow_mut().next(), 1);
}

#[test]
fn copy_pattern() {
    let model = rvs::parse(
        Default::default(),
    "a = Pattern(0, 1, 2, 3); b = a.copy;"
        ).unwrap();

    let a = model.get_variable_by_name("a").unwrap();
    let b = model.get_variable_by_name("b").unwrap();

    assert_eq!(b.borrow_mut().next(), 0);
    assert_eq!(a.borrow_mut().next(), 0);
    assert_eq!(b.borrow_mut().next(), 1);
    assert_eq!(a.borrow_mut().next(), 1);
    assert_eq!(b.borrow_mut().next(), 2);
    assert_eq!(a.borrow_mut().next(), 2);
}

#[test]
fn prev() {
    let model = rvs::parse(
        Default::default(),
    "a = Pattern(0, 1, 2, 3); b = a.prev;"
        ).unwrap();

    let a = model.get_variable_by_name("a").unwrap();
    let b = model.get_variable_by_name("b").unwrap();

    assert_eq!(b.borrow_mut().next(), 0);
    assert_eq!(a.borrow_mut().next(), 0);
    assert_eq!(b.borrow_mut().next(), 0);
    assert_eq!(a.borrow_mut().next(), 1);
    assert_eq!(b.borrow_mut().next(), 1);
    assert_eq!(a.borrow_mut().next(), 2);
    assert_eq!(b.borrow_mut().next(), 2);
}
