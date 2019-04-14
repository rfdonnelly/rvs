extern crate rvs;

mod util;
use crate::util::*;

use std::collections::HashSet;

#[test]
fn reverse_limits() {
    let a = expr_to_var("[1, 0]").unwrap();
    let mut a = a.borrow_mut();

    let expected: HashSet<u32> = [0, 1].iter().cloned().collect();
    let actual: HashSet<u32> = (0..10).map(|_| a.next()).collect();

    assert_eq!(expected, actual);
}

#[test]
fn equal_limits() {
    let a = expr_to_var("[1, 1]").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<u32> = (0..10).map(|_| 1).collect();
    let actual: Vec<u32> = (0..10).map(|_| a.next()).collect();

    assert_eq!(expected, actual);
}

#[test]
fn done_after_each_next() {
    let a = expr_to_var("[0, 8]").unwrap();
    let mut a = a.borrow_mut();

    assert_eq!(a.done(), false);

    let expected: Vec<bool> = (0..20).map(|_| true).collect();
    let actual: Vec<bool> = (0..20)
        .map(|_| {
            a.next();
            a.done()
        })
        .collect();

    assert_eq!(expected, actual);
}
