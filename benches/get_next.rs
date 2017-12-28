#![feature(test)]
extern crate test;

use test::Bencher;

extern crate rvs;

use rvs::parse;
use rvs::types::Context;

#[bench]
fn basic(b: &mut Bencher) {
    let mut context = Context::new();
    let mut source = String::new();
    let iter = 64*1024;
    for i in 0..iter {
        source = format!("{}a{} = {};\n", source, i, i);
    }
    parse(&source, &mut context).unwrap();

    b.iter(||
           for i in 0..iter {
               let name = format!("a{}", i);
               let rv = context.get(&name).unwrap();
               assert_eq!(rv.borrow_mut().next(), i);
           });
}
