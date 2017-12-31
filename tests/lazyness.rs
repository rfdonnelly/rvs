extern crate rvs;

use rvs::Context;

#[test]
fn function_arguments() {
    let mut context = Context::new();
    rvs::parse("b=0; a=Sequence(b); b=10;", &mut context).unwrap();
    rvs::transform(&mut context).unwrap();

    let a = context.get("a").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<u32> = (0..10).collect();
    let actual: Vec<u32> = (0..10).map(|_| {
        a.next()
    }).collect();

    assert_eq!(expected, actual);
}
