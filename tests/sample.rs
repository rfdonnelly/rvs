extern crate rvs;

use std::collections::HashSet;

mod util;
use util::*;

#[test]
fn basic() {
    let a = expr_to_var("Sample(1, 2, 4, 8)").unwrap();
    let mut a = a.borrow_mut();

    let expected: HashSet<u32> = [1, 2, 4, 8].iter().cloned().collect();
    let actual: HashSet<u32> = (0..16).map(|_| a.next()).collect();

    assert_eq!(expected, actual);
}

#[test]
fn selects_another_subexpr_when_current_subexpr_done() {
    let a = expr_to_var("Sample(Pattern(0, 1), Pattern(2, 3))").unwrap();
    let mut a = a.borrow_mut();

    for _ in 0..100 {
        let value = a.next();

        match value {
            0 => assert_eq!(a.next(), 1),
            2 => assert_eq!(a.next(), 3),
            value => assert!(value == 0 || value == 2),
        }
    }
}

#[test]
fn done_when_sub_expr_done() {
    let a = expr_to_var("Sample(Pattern(0, 1), Pattern(2, 3))").unwrap();
    let mut a = a.borrow_mut();

    assert_eq!(a.done(), false);

    let expected: Vec<bool> = vec![false, true].into_iter().cycle().take(32).collect();
    let actual: Vec<bool> = (0..expected.len())
        .map(|_| {
            a.next();
            a.done()
        })
        .collect();

    assert_eq!(expected, actual);
}
