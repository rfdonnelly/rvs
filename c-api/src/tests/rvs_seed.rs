use super::*;

fn next(seed: u32, s: &str) -> u32 {
    let context = rvs_context_new();
    let error = rvs_error_new();
    rvs_seed(context, seed);

    let s = format!("a = {};", s);
    rvs_parse(context, CString::new(s).unwrap().as_ptr(), error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    let handle = rvs_find(context, CString::new("a").unwrap().as_ptr());
    assert!(handle != 0);

    let value = rvs_next(context, handle);

    rvs_error_free(error);
    rvs_context_free(context);

    value
}

#[test]
fn basic() {
    let s = "[0, 0xffff_ffff]";

    let seed0_value0 = next(0, s);
    let seed1_value0 = next(1, s);
    let seed0_value1 = next(0, s);
    let seed1_value1 = next(1, s);

    assert!(seed0_value0 != seed1_value0);
    assert_eq!(seed0_value0, seed0_value1);
    assert_eq!(seed1_value0, seed1_value1);
}
