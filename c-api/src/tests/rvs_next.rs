use super::*;

#[test]
fn found() {
    let error = rvs_error_new();
    let context = rvs_context_new(CString::new("").unwrap().as_ptr(), 0, error);
    assert!(!rvs_error_test(error));

    rvs_parse(context, CString::new("a=5;").unwrap().as_ptr(), error);
    assert!(!rvs_error_test(error));

    let model = rvs_model_new();
    rvs_transform(context, model, error);
    assert!(!rvs_error_test(error));

    let handle = rvs_get(model, CString::new("a").unwrap().as_ptr());
    assert!(handle != 0);

    assert_eq!(rvs_done(model, handle), false);
    assert_eq!(rvs_prev(model, handle), 5);
    let value = rvs_next(model, handle);
    assert_eq!(value, 5);
    assert_eq!(rvs_done(model, handle), true);
    assert_eq!(rvs_prev(model, handle), 5);

    rvs_error_free(error);
    rvs_model_free(model);
}

#[test]
fn not_found() {
    let error = rvs_error_new();
    let context = rvs_context_new(CString::new("").unwrap().as_ptr(), 0, error);
    assert!(!rvs_error_test(error));

    rvs_parse(context, CString::new("a=5;").unwrap().as_ptr(), error);
    assert!(!rvs_error_test(error));

    let model = rvs_model_new();
    rvs_transform(context, model, error);
    assert!(!rvs_error_test(error));

    let handle = 2;
    let value = rvs_next(model, handle);
    assert_eq!(value, 0);

    rvs_model_free(model);
}
