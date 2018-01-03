use super::*;

#[test]
fn not_found() {
    let context = rvs_context_new();

    let handle = rvs_find(context, CString::new("a").unwrap().as_ptr());
    assert_eq!(handle, 0);

    rvs_context_free(context);
}

#[test]
fn found() {
    let context = rvs_context_new();
    let error = rvs_error_new();

    rvs_parse(context, CString::new("a=5;").unwrap().as_ptr(), error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    rvs_transform(context, error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    let handle = rvs_find(context, CString::new("a").unwrap().as_ptr());
    assert!(handle != 0);

    rvs_error_free(error);
    rvs_context_free(context);
}
