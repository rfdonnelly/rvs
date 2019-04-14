extern crate rvs;

mod util;
use crate::util::*;

#[test]
fn yields_a_repeating_pattern() {
    let a = expr_to_var("Pattern(0, 1, 2, 3)").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<(u32, bool)> = (0..4)
        .zip(vec![false, false, false, true].into_iter())
        .cycle()
        .take(16)
        .collect();

    let actual: Vec<(u32, bool)> = (0..16).map(|_| (a.next(), a.done())).collect();

    assert_eq!(actual, expected);
}

#[test]
fn selects_another_subexpr_when_current_subexpr_done() {
    let a = expr_to_var("Pattern(Pattern(0, 1), Pattern(2, 3))").unwrap();
    let mut a = a.borrow_mut();

    assert_eq!(a.done(), false);

    let expected: Vec<(u32, bool)> = (0..4)
        .zip(vec![false, false, false, true].into_iter())
        .cycle()
        .take(16)
        .collect();

    let actual: Vec<(u32, bool)> = (0..16).map(|_| (a.next(), a.done())).collect();

    assert_eq!(actual, expected);
}
