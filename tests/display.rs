extern crate rvs;

use rvs::Context;

#[test]
fn multiple() {
    let mut context = Context::new();
    rvs::parse("a=[0,1];b=[2,3];", &mut context).unwrap();
    assert_eq!(context.to_string(), "a = [0x0, 0x1];\nb = [0x2, 0x3];\n");
}

#[test]
fn precendence() {
    let mut context = Context::new();
    rvs::parse("a = (10 + 6) * 8;", &mut context).unwrap();
    assert_eq!(context.to_string(), "a = ((0xa + 0x6) * 0x8);\n");
}
