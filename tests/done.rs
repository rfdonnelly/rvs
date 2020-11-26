mod util;
use crate::util::*;

use std::iter;

#[test]
fn done_defaults_to_false() {
    let a = expr_to_var("Done(1)").unwrap();
    let a = a.borrow();

    assert_eq!(a.done(), false);
}

#[test]
fn done_causes_advance_to_next_subexpr() {
    let a = expr_to_var("Pattern(Done(Pattern(0, 1, 2, 3)), Done(Pattern(4, 5, 6, 7)))").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<(u32, bool)> = (0..8)
        .zip(vec![false, true].into_iter().cycle().take(8))
        .cycle()
        .take(32)
        .map(|(value, done)| {
            let value = if value % 2 == 0 {
                value / 2
            } else {
                value / 2 + 4
            };
            (value, done)
        })
        .collect();
    let actual: Vec<(u32, bool)> = (0..32).map(|_| (a.next(), a.done())).collect();

    assert_eq!(expected, actual);
}

#[test]
fn always_done() {
    let a = expr_to_var("Done(Pattern(0, 1, 2, 3))").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<(u32, bool)> = (0..4)
        .zip(iter::repeat(true).take(4))
        .cycle()
        .take(32)
        .collect();
    let actual: Vec<(u32, bool)> = (0..32).map(|_| (a.next(), a.done())).collect();

    assert_eq!(expected, actual);
}
