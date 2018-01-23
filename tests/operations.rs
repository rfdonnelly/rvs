extern crate rvs;

mod util;
use util::*;

use std::iter;

#[test]
fn yields_sum() {
    let a = expr_to_var("Pattern(0, 1, 2) + Pattern(0, 1, 2, 3)").unwrap();
    let mut a = a.borrow_mut();

    let l = (0..3).cycle().take(24);
    let r = (0..4).cycle().take(24);
    let expected: Vec<(u32, bool)> = l.zip(r)
        .map(|(l, r)| l + r)
        .zip(
            iter::repeat(false)
                .take(3)
                .chain(iter::repeat(true).take(21)),
        )
        .collect();

    let actual: Vec<(u32, bool)> = (0..24).map(|_| (a.next(), a.done())).collect();

    assert_eq!(expected, actual);
}

#[test]
fn overflow_add() {
    let a = expr_to_var("~0 + ~0").unwrap();
    let mut a = a.borrow_mut();

    assert_eq!(0xffff_fffe, a.next());
}

#[test]
fn overflow_sub() {
    let a = expr_to_var("0 - 1").unwrap();
    let mut a = a.borrow_mut();

    assert_eq!(0xffff_ffff, a.next());
}

#[test]
fn overflow_mul() {
    let a = expr_to_var("~0 * 2").unwrap();
    let mut a = a.borrow_mut();

    assert_eq!(0xffff_fffe, a.next());
}

#[test]
fn overflow_shl() {
    let a = expr_to_var("1 << 33").unwrap();
    let mut a = a.borrow_mut();

    assert_eq!(2, a.next());
}

#[test]
fn overflow_shr() {
    let a = expr_to_var("1 >> 33").unwrap();
    let mut a = a.borrow_mut();

    assert_eq!(0, a.next());
}

#[test]
fn overflow_neg() {
    let a = expr_to_var("-0").unwrap();
    let mut a = a.borrow_mut();

    assert_eq!(0, a.next());
}
