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
    let context = rvs_context_new();
    let error = rvs_error_new();

    let search_path = ::std::env::current_dir()
        .unwrap()
        .join("../examples");
    let search_path = search_path
        .to_str()
        .unwrap();
    rvs_search_path(context, CString::new(search_path).unwrap().as_ptr(), error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    rvs_parse(context, CString::new("import '../examples/import.rvs'").unwrap().as_ptr(), error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    rvs_transform(context, error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    assert_eq!(next_by_name(context, "a"), 5);
    assert_eq!(next_by_name(context, "b"), 1);

    rvs_error_free(error);
    rvs_context_free(context);
}

#[test]
fn basic() {
    let context = rvs_context_new();
    let error = rvs_error_new();

    rvs_parse(context, CString::new("a=5;").unwrap().as_ptr(), error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    rvs_transform(context, error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    let variable = unsafe { (*context).variables.get("a").unwrap() };
    let value = variable.borrow_mut().next();
    assert_eq!(value, 5);

    rvs_error_free(error);
    rvs_context_free(context);
}

#[test]
fn range() {
    let context = rvs_context_new();
    let error = rvs_error_new();

    rvs_parse(context, CString::new("a=[0,1];").unwrap().as_ptr(), error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    rvs_transform(context, error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    let variable = unsafe { (*context).variables.get("a").unwrap() };
    let value = variable.borrow_mut().next();
    assert!(value == 0 || value == 1);

    rvs_error_free(error);
    rvs_context_free(context);
}

#[test]
fn parse_error() {
    let context = rvs_context_new();
    let error = rvs_error_new();

    rvs_parse(context, CString::new("a = 1;\n1 = b;").unwrap().as_ptr(), error);
    // FIXME: Check error message
    // println!("{}", unsafe { *error });
    // assert_eq!(rvs_error_code(error), ErrorKind::Parse(rvs::ParseError::new()).code());
    assert!(rvs_error_code(error) != ErrorKind::None.code());

    rvs_error_free(error);
    rvs_context_free(context);
}

#[test]
fn file() {
    let context = rvs_context_new();
    let error = rvs_error_new();

    rvs_parse(context, CString::new("../examples/basic.rvs;b = 3").unwrap().as_ptr(), error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    rvs_transform(context, error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    let handle = rvs_find(context, CString::new("a").unwrap().as_ptr());
    assert!(handle != 0);

    let value = rvs_next(context, handle);
    assert_eq!(value, 5);

    let handle = rvs_find(context, CString::new("b").unwrap().as_ptr());
    assert!(handle != 0);

    let value = rvs_next(context, handle);
    assert_eq!(value, 3);

    rvs_error_free(error);
    rvs_context_free(context);
}

#[test]
fn override_rv() {
    let context = rvs_context_new();
    let error = rvs_error_new();

    rvs_parse(context, CString::new("a = 0;a = 1").unwrap().as_ptr(), error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    rvs_parse(context, CString::new("a = 2").unwrap().as_ptr(), error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    rvs_transform(context, error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    let handle = rvs_find(context, CString::new("a").unwrap().as_ptr());
    assert!(handle != 0);

    let value = rvs_next(context, handle);
    assert_eq!(value, 2);

    rvs_error_free(error);
    rvs_context_free(context);
}
