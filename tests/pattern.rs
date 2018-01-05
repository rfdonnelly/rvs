extern crate rvs;

mod util;
use util::*;

#[test]
fn yields_a_repeating_pattern() {
    let a = expr_to_var("Pattern(0, 1, 2, 3)").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<u32> = (0..4)
        .cycle().take(16)
        .collect();

    let actual: Vec<u32> = (0..16)
        .map(|_| a.next())
        .collect();

    assert_eq!(actual, expected);
}
