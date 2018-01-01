use super::*;

fn next(seed: u32, s: &str) -> u32 {
    let error = rvs_error_new();
    let context = rvs_context_new(CString::new("").unwrap().as_ptr(), seed, error);
    assert!(!rvs_error_test(error));

    let s = format!("a = {};", s);
    rvs_parse(context, CString::new(s).unwrap().as_ptr(), error);
    assert!(!rvs_error_test(error));

    let model = rvs_transform(context, error);
    assert!(!rvs_error_test(error));

    let handle = rvs_get(model, CString::new("a").unwrap().as_ptr());
    assert!(handle != 0);

    let value = rvs_next(model, handle);

    rvs_error_free(error);
    rvs_model_free(model);

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
