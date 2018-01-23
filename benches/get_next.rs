#![feature(test)]
extern crate test;

use test::Bencher;

extern crate rvs;

#[bench]
fn basic(b: &mut Bencher) {
    let mut source = String::new();
    let iter = 64 * 1024;
    for i in 0..iter {
        source = format!("{}a{} = {};\n", source, i, i);
    }
    let search_path = Default::default();
    let model = rvs::parse(search_path, &source).unwrap();

    b.iter(|| {
        for i in 0..iter {
            let name = format!("a{}", i);
            let variable = model.get_variable_by_name(&name).unwrap();
            assert_eq!(variable.borrow_mut().next(), i);
        }
    });
}
