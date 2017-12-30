extern crate rvs;

use rvs::Context;

#[test]
fn count() {
    let mut context = Context::new();
    rvs::parse("a = Sequence(10);", &mut context).unwrap();

    let a = context.get("a").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<u32> = (0..10).collect();
    let actual: Vec<u32> = (0..10).map(|_| {
        a.next()
    }).collect();

    assert_eq!(expected, actual);
}

#[test]
fn offset_count() {
    let mut context = Context::new();
    rvs::parse("a = Sequence(10, 10);", &mut context).unwrap();

    let a = context.get("a").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<u32> = (10..20).collect();
    let actual: Vec<u32> = (0..10).map(|_| {
        a.next()
    }).collect();

    assert_eq!(expected, actual);
}

#[test]
fn offset_increment_count() {
    let mut context = Context::new();
    rvs::parse("a = Sequence(0, 4, 10);", &mut context).unwrap();

    let a = context.get("a").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<u32> = (0..10).map(|i| {
        i * 4
    }).collect();
    let actual: Vec<u32> = (0..10).map(|_| {
        a.next()
    }).collect();

    assert_eq!(expected, actual);
}

#[test]
#[should_panic]
fn zero_count() {
    let mut context = Context::new();
    rvs::parse("a = Sequence(0);", &mut context).unwrap();

    let a = context.get("a").unwrap();
    let mut a = a.borrow_mut();

    assert_eq!(a.next(), 0);
}
