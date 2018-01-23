extern crate rvs;

#[test]
fn multiple() {
    let model = rvs::parse(Default::default(), "a=[0,1];b=[2,3];").unwrap();

    assert_eq!(model.to_string(), "a = [0x0, 0x1];\nb = [0x2, 0x3];\n");
}

#[test]
fn precendence() {
    let model = rvs::parse(Default::default(), "a = (10 + 6) * 8;").unwrap();

    assert_eq!(model.to_string(), "a = ((0xa + 0x6) * 0x8);\n");
}
