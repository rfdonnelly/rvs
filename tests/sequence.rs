extern crate rvs;

mod util;
use util::*;

#[test]
fn count() {
    let a = expr_to_var("Sequence(4)").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<u32> = (0..4)
        .cycle().take(16)
        .collect();
    let actual: Vec<u32> = (0..16)
        .map(|_| a.next())
        .collect();

    assert_eq!(expected, actual);
}

#[test]
fn offset_count() {
    let a = expr_to_var("Sequence(10, 4)").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<u32> = (10..14)
        .cycle().take(16)
        .collect();
    let actual: Vec<u32> = (0..16)
        .map(|_| a.next())
        .collect();

    assert_eq!(expected, actual);
}

#[test]
fn offset_increment_count() {
    let a = expr_to_var("Sequence(0, 4, 4)").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<u32> = (0..4)
        .cycle().take(16)
        .map(|i| i * 4)
        .collect();
    let actual: Vec<u32> = (0..16)
        .map(|_| a.next())
        .collect();

    assert_eq!(expected, actual);
}

#[test]
#[should_panic]
fn zero_count() {
    let a = expr_to_var("Sequence(0)").unwrap();
    let mut a = a.borrow_mut();

    assert_eq!(a.next(), 0);
}
