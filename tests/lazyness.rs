extern crate rvs;

#[test]
fn type_arguments() {
    let model = rvs::parse(
        Default::default(),
        "b=0; a=Sequence(b); b=10;"
        ).unwrap();

    let a = model.get_variable_by_name("a").unwrap();
    let mut a = a.borrow_mut();

    let expected: Vec<u32> = (0..10)
        .collect();
    let actual: Vec<u32> = (0..10)
        .map(|_| a.next())
        .collect();

    assert_eq!(expected, actual);
}
