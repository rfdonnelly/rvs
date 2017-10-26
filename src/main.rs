extern crate rand;

mod ast;
mod sequence;
mod grammar;

use grammar::*;

fn main() {
}

fn eval(expr: &str) -> u32 {
    match infix_arith(expr) {
        Ok(ast) => ast.eval(),
        Err(_) => panic!("Could not parse: '{}'", expr),
    }
}

mod tests {
    mod eval {
        use super::super::*;

        #[test]
        fn basic() {
            assert_eq!(eval("1+2*3"), 7);
        }
    }
}

