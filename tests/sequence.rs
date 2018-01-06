extern crate rvs;

mod util;
use util::*;

#[test]
fn count() {
    let a = expr_to_var("Sequence(4)").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<(u32, bool)> = (0..4)
        .zip(vec![false, false, false, true].into_iter())
        .cycle().take(16)
        .collect();
    let actual: Vec<(u32, bool)> = (0..16)
        .map(|_| (a.next(), a.done()))
        .collect();

    assert_eq!(expected, actual);
}

#[test]
fn offset_count() {
    let a = expr_to_var("Sequence(10, 4)").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<(u32, bool)> = (10..14)
        .zip(vec![false, false, false, true].into_iter())
        .cycle().take(16)
        .collect();
    let actual: Vec<(u32, bool)> = (0..16)
        .map(|_| (a.next(), a.done()))
        .collect();

    assert_eq!(expected, actual);
}

#[test]
fn offset_increment_count() {
    let a = expr_to_var("Sequence(0, 4, 4)").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<(u32, bool)> = (0..4)
        .zip(vec![false, false, false, true].into_iter())
        .cycle().take(16)
        .map(|(i, done)| (i * 4, done))
        .collect();
    let actual: Vec<(u32, bool)> = (0..16)
        .map(|_| (a.next(), a.done()))
        .collect();

    assert_eq!(expected, actual);
}

#[test]
fn decrement() {
    let a = expr_to_var("Sequence(3, -1, 4)").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<(u32, bool)> = (0..4)
        .rev()
        .zip(vec![false, false, false, true].into_iter())
        .cycle().take(16)
        .collect();
    let actual: Vec<(u32, bool)> = (0..16)
        .map(|_| (a.next(), a.done()))
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
