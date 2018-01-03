extern crate rvs_parser;

mod utils;
use utils::*;

#[test]
fn ast() {
    assert_eq!(
        parse("a = [1,2];"),
        "[Variable(\"a\", Function(Range, [Number(1), Number(2)]))]");
}
