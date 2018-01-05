extern crate rvs;

use std::collections::HashSet;

mod util;
use util::*;

#[test]
fn basic() {
    let a = expr_to_var("Sample(1, 2, 4, 8)").unwrap();
    let mut a = a.borrow_mut();

    let expected: HashSet<u32> =
        [1, 2, 4, 8].iter().cloned().collect();
    let actual: HashSet<u32> = (0..16)
        .map(|_| a.next())
        .collect();

    assert_eq!(expected, actual);
}

    }

    assert_eq!(expected, actual);
}
