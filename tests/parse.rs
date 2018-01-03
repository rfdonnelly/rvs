extern crate rvs;

#[test]
fn basic() {
    let model = rvs::parse(
        Default::default(),
        "a=[0,1];\nb=2;"
        ).unwrap();

    let a = model.get_variable_by_name("a").unwrap();
    let result = a.borrow_mut().next();
    assert!(result == 0 || result == 1);

    let b = model.get_variable_by_name("b").unwrap();
    let result = b.borrow_mut().next();
    assert_eq!(result, 2);
}
