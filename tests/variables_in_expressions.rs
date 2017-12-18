extern crate rvs;

use rvs::parse;
use rvs::types::Context;

#[test]
fn next() {
    let mut context = Context::new();
    parse("a = 1; b = a;", &mut context).unwrap();

    let b = context.get("b").unwrap();

    assert_eq!(b.borrow_mut().next(&context), 1);
}

/// Verifies that the underlying variable's state is advanced
#[test]
fn next_pattern() {
    let mut context = Context::new();
    parse("a = Pattern(0, 1, 2, 3); b = a;", &mut context).unwrap();

    let a = context.get("a").unwrap();
    let b = context.get("b").unwrap();

    assert_eq!(b.borrow_mut().next(&context), 0);
    assert_eq!(a.borrow_mut().next(&context), 1);
    assert_eq!(b.borrow_mut().next(&context), 2);
    assert_eq!(a.borrow_mut().next(&context), 3);
}

#[test]
fn copy() {
    let mut context = Context::new();
    parse("a = 1; b = a.copy;", &mut context).unwrap();

    let b = context.get("b").unwrap();

    assert_eq!(b.borrow_mut().next(&context), 1);
}

#[test]
fn copy_pattern() {
    let mut context = Context::new();
    parse("a = Pattern(0, 1, 2, 3); b = a.copy;", &mut context).unwrap();

    let a = context.get("a").unwrap();
    let b = context.get("b").unwrap();

    assert_eq!(b.borrow_mut().next(&context), 0);
    assert_eq!(a.borrow_mut().next(&context), 0);
    assert_eq!(b.borrow_mut().next(&context), 1);
    assert_eq!(a.borrow_mut().next(&context), 1);
    assert_eq!(b.borrow_mut().next(&context), 2);
    assert_eq!(a.borrow_mut().next(&context), 2);
}

#[test]
fn prev() {
    let mut context = Context::new();
    parse("a = Pattern(0, 1, 2, 3); b = a.prev;", &mut context).unwrap();

    let a = context.get("a").unwrap();
    let b = context.get("b").unwrap();

    assert_eq!(b.borrow_mut().next(&context), 0);
    assert_eq!(a.borrow_mut().next(&context), 0);
    assert_eq!(b.borrow_mut().next(&context), 0);
    assert_eq!(a.borrow_mut().next(&context), 1);
    assert_eq!(b.borrow_mut().next(&context), 1);
    assert_eq!(a.borrow_mut().next(&context), 2);
    assert_eq!(b.borrow_mut().next(&context), 2);
}
