extern crate rvs;

use rvs::parse;
use rvs::types::Context;

#[test]
fn multiple() {
    let mut context = Context::new();
    assert!(parse("a=[0,1];b=[2,3];", &mut context).is_ok());
    assert_eq!(context.to_string(), "a = [0x0, 0x1];\nb = [0x2, 0x3];\n");
}

#[test]
fn precendence() {
    let mut context = Context::new();
    assert!(parse("a = (10 + 6) * 8;", &mut context).is_ok());
    assert_eq!(context.to_string(), "a = ((0xa + 0x6) * 0x8);\n");
}
