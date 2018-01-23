extern crate rvs;

mod util;
use util::*;

use std::collections::HashMap;

#[test]
fn distribution() {
    let a = expr_to_var("{10: 0, 90: 1}").unwrap();
    let mut a = a.borrow_mut();

    let mut actual: HashMap<u32, u32> = HashMap::new();

    for _ in 0..1000 {
        let entry = actual.entry(a.next()).or_insert(0);
        *entry += 1;
    }

    println!("expected:{{0: 100, 1: 900}} actual:{:?}", actual);
    assert!(actual[&0] >= 100 - 20 && actual[&0] <= 100 + 20);
    assert!(actual[&1] >= 900 - 20 && actual[&1] <= 900 + 20);
}

#[test]
fn selects_another_subexpr_when_current_subexpr_done() {
    let a = expr_to_var("{1: Pattern(0, 1), 1: Pattern(2, 3)}").unwrap();
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
    let a = expr_to_var("{1: Pattern(0, 1), 1: Pattern(2, 3)}").unwrap();
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
