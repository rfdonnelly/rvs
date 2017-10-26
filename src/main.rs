extern crate rand;

mod ast;
mod sequence;
mod grammar;

use grammar::*;

fn main() {
}

fn eval_by_ast(expr: &str) -> u32 {
    match infix_arith(expr) {
        Ok(ast) => ast.eval(),
        Err(_) => panic!("Could not parse: '{}'", expr),
    }
}

mod tests {
    mod eval_by_ast {
        use super::super::*;

        #[test]
        fn basic() {
            assert_eq!(eval_by_ast("1+2*3"), 7);
        }
    }
}

