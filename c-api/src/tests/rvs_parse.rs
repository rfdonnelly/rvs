use super::*;

// FIXME Cannot test this due to error parameter being mutable while std::ptr::null() is
// not.
// #[test]
// fn null_error_struct() {
//     use std::ptr;
//
//     let context = rvs_context_new();
//     rvs_parse(context, CString::new("a=5;").unwrap().as_ptr(), ptr::null());
//     assert_eq!(next_by_name(context, "a"), 5);
// }

#[test]
fn import() {
    let search_path = ::std::env::current_dir()
        .unwrap()
        .join("../examples");
    let search_path = search_path
        .to_str()
        .unwrap();

    let error = rvs_error_new();
    let context = rvs_context_new(CString::new(search_path).unwrap().as_ptr(), 0, error);
    assert!(!rvs_error_test(error));

    rvs_parse(context, CString::new("import import;").unwrap().as_ptr(), error);
    assert!(!rvs_error_test(error));

    let model = rvs_model_new();
    rvs_transform(context, model, error);
    assert!(!rvs_error_test(error));

    assert_eq!(next_by_name(model, "a"), 5);
    assert_eq!(next_by_name(model, "b"), 1);

    rvs_error_free(error);
    rvs_model_free(model);
}

#[test]
fn basic() {
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

    let value = rvs_next(model, handle);
    assert_eq!(value, 5);

    rvs_error_free(error);
    rvs_model_free(model);
}

#[test]
fn range() {
    let error = rvs_error_new();
    let context = rvs_context_new(CString::new("").unwrap().as_ptr(), 0, error);
    assert!(!rvs_error_test(error));

    rvs_parse(context, CString::new("a=[0,1];").unwrap().as_ptr(), error);
    assert!(!rvs_error_test(error));

    let model = rvs_model_new();
    rvs_transform(context, model, error);
    assert!(!rvs_error_test(error));

    let handle = rvs_get(model, CString::new("a").unwrap().as_ptr());
    assert!(handle != 0);

    let value = rvs_next(model, handle);
    assert!(value == 0 || value == 1);

    rvs_error_free(error);
    rvs_model_free(model);
}

#[test]
fn parse_error() {
    let error = rvs_error_new();
    let context = rvs_context_new(CString::new("").unwrap().as_ptr(), 0, error);
    assert!(!rvs_error_test(error));

    rvs_parse(context, CString::new("a = 1;\n1 = b;").unwrap().as_ptr(), error);
    // FIXME: Check error message
    // println!("{}", unsafe { *error });
    // assert_eq!(rvs_error_code(error), ErrorKind::Parse(rvs::ParseError::new()).code());
    assert!(rvs_error_test(error));

    rvs_error_free(error);
    rvs_context_free(context);
}

#[test]
fn file() {
    let error = rvs_error_new();
    let context = rvs_context_new(CString::new("").unwrap().as_ptr(), 0, error);
    assert!(!rvs_error_test(error));

    rvs_parse(context, CString::new("../examples/basic.rvs;b = 3").unwrap().as_ptr(), error);
    assert!(!rvs_error_test(error));

    let model = rvs_model_new();
    rvs_transform(context, model, error);
    assert!(!rvs_error_test(error));

    let handle = rvs_get(model, CString::new("a").unwrap().as_ptr());
    assert!(handle != 0);

    let value = rvs_next(model, handle);
    assert_eq!(value, 5);

    let handle = rvs_get(model, CString::new("b").unwrap().as_ptr());
    assert!(handle != 0);

    let value = rvs_next(model, handle);
    assert_eq!(value, 3);

    rvs_error_free(error);
    rvs_model_free(model);
}

#[test]
fn override_rv() {
    let error = rvs_error_new();
    let context = rvs_context_new(CString::new("").unwrap().as_ptr(), 0, error);
    assert!(!rvs_error_test(error));

    rvs_parse(context, CString::new("a = 0;a = 1").unwrap().as_ptr(), error);
    assert!(!rvs_error_test(error));

    rvs_parse(context, CString::new("a = 2").unwrap().as_ptr(), error);
    assert!(!rvs_error_test(error));

    let model = rvs_model_new();
    rvs_transform(context, model, error);
    assert!(!rvs_error_test(error));

    let handle = rvs_get(model, CString::new("a").unwrap().as_ptr());
    assert!(handle != 0);

    let value = rvs_next(model, handle);
    assert_eq!(value, 2);

    rvs_error_free(error);
    rvs_model_free(model);
}
