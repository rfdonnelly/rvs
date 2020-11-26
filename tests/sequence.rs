mod util;
use crate::util::*;

#[test]
fn last() {
    let a = expr_to_var("Sequence(3)").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<(u32, bool)> = (0..4)
        .zip(vec![false, false, false, true].into_iter())
        .cycle()
        .take(16)
        .collect();
    let actual: Vec<(u32, bool)> = (0..16).map(|_| (a.next(), a.done())).collect();

    assert_eq!(expected, actual);
}

#[test]
fn first_last() {
    let (first, last) = (10, 13);

    let a = expr_to_var(format!("Sequence({}, {})", first, last)).unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<(u32, bool)> = (first..last + 1)
        .zip(vec![false, false, false, true].into_iter())
        .cycle()
        .take(16)
        .collect();
    let actual: Vec<(u32, bool)> = (0..16).map(|_| (a.next(), a.done())).collect();

    assert_eq!(expected, actual);
}

#[test]
fn first_last_increment() {
    let a = expr_to_var("Sequence(0, 12, 4)").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<(u32, bool)> = (0..4)
        .zip(vec![false, false, false, true].into_iter())
        .cycle()
        .take(16)
        .map(|(i, done)| (i * 4, done))
        .collect();
    let actual: Vec<(u32, bool)> = (0..16).map(|_| (a.next(), a.done())).collect();

    assert_eq!(expected, actual);
}

#[test]
fn decrement() {
    let (first, last) = (3, 0);
    let a = expr_to_var(format!("Sequence({}, {}, -1)", first, last)).unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<(u32, bool)> = (last..first + 1)
        .rev()
        .zip(vec![false, false, false, true].into_iter())
        .cycle()
        .take(16)
        .collect();
    let actual: Vec<(u32, bool)> = (0..16).map(|_| (a.next(), a.done())).collect();

    assert_eq!(expected, actual);
}

#[test]
fn args_evaluated_every_cycle() {
    let a = expr_to_var("Sequence(Pattern(2, 4), Pattern(4, 16), Pattern(2, 4))").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<(u32, bool)> = vec![
        (2, false),
        (4, true),
        (4, false),
        (8, false),
        (12, false),
        (16, true),
    ].into_iter()
        .cycle()
        .take(16)
        .collect();
    let actual: Vec<(u32, bool)> = (0..16).map(|_| (a.next(), a.done())).collect();

    assert_eq!(expected, actual);
}

#[test]
fn increment_skips_last() {
    let a = expr_to_var("Sequence(0, 3, 2)").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<(u32, bool)> = vec![(0, false), (2, true)]
        .into_iter()
        .cycle()
        .take(16)
        .collect();
    let actual: Vec<(u32, bool)> = (0..16).map(|_| (a.next(), a.done())).collect();

    assert_eq!(expected, actual);
}

#[test]
fn negative_increment_skips_last() {
    let a = expr_to_var("Sequence(13, 10, -2)").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<(u32, bool)> = vec![(13, false), (11, true)]
        .into_iter()
        .cycle()
        .take(16)
        .collect();
    let actual: Vec<(u32, bool)> = (0..16).map(|_| (a.next(), a.done())).collect();

    assert_eq!(expected, actual);
}

#[test]
fn zero_last() {
    let a = expr_to_var("Sequence(0)").unwrap();
    let mut a = a.borrow_mut();

    assert_eq!(a.next(), 0);
}

#[test]
#[should_panic(expected = "the increment sub-expression `Pattern(0x1, 0x0, )` returned 0 in the expression `Sequence(0x0, 0x9, Pattern(0x1, 0x0, ))`")]
fn zero_increment() {
    let a = expr_to_var("Sequence(0, 9, Pattern(1, 0))").unwrap();
    let mut a = a.borrow_mut();

    (0..20).for_each(|_| {
        a.next();
    });
}
