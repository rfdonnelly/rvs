extern crate rvs;

use rvs::parse;
use rvs::types::Context;

#[test]
fn basic() {
    let mut context = Context::new();
    assert!(parse("a=[0,1];\nb=2;", &mut context).is_ok());

    {
        let a = context.get("a").unwrap();
        let result = a.borrow_mut().next(&context);
        assert!(result == 0 || result == 1);
    }

    {
        let b = context.get("b").unwrap();
        let result = b.borrow_mut().next(&context);
        assert_eq!(result, 2);
    }
}
