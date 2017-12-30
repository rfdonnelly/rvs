extern crate rvs;

use rvs::Context;

#[test]
fn basic() {
    let mut context = Context::new();
    rvs::parse("a=[0,1];\nb=2;", &mut context).unwrap();

    {
        let a = context.get("a").unwrap();
        let result = a.borrow_mut().next();
        assert!(result == 0 || result == 1);
    }

    {
        let b = context.get("b").unwrap();
        let result = b.borrow_mut().next();
        assert_eq!(result, 2);
    }
}
