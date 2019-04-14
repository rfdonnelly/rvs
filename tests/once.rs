extern crate rvs;

mod util;
use crate::util::*;

use std::iter;

#[test]
fn done_defaults_to_false() {
    let a = expr_to_var("Once([0, 1])").unwrap();
    let a = a.borrow();

    assert_eq!(a.done(), false);
}

#[test]
fn yields_constant_value_and_always_done() {
    let a = expr_to_var("Once(Pattern(1, 2, 3))").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<(u32, bool)> = iter::repeat((1, true)).take(32).collect();
    let actual: Vec<(u32, bool)> = (0..expected.len()).map(|_| (a.next(), a.done())).collect();

    assert_eq!(expected, actual);
}
