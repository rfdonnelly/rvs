use super::*;

#[test]
fn not_found() {
    let error = rvs_error_new();
    let context = rvs_context_new(CString::new("").unwrap().as_ptr(), 0, error);
    assert!(!rvs_error_test(error));

    let model = rvs_transform(context, error);
    assert!(!rvs_error_test(error));

    let handle = rvs_get(model, CString::new("a").unwrap().as_ptr());
    assert_eq!(handle, 0);

    rvs_model_free(model);
}

#[test]
fn found() {
    let error = rvs_error_new();
    let context = rvs_context_new(CString::new("").unwrap().as_ptr(), 0, error);
    assert!(!rvs_error_test(error));

    rvs_parse(context, CString::new("a=5;").unwrap().as_ptr(), error);
    assert!(!rvs_error_test(error));

    let model = rvs_transform(context, error);
    assert!(!rvs_error_test(error));

    let handle = rvs_get(model, CString::new("a").unwrap().as_ptr());
    assert!(handle != 0);

    rvs_error_free(error);
    rvs_model_free(model);
}
