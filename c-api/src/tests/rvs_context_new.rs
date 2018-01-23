use super::*;

#[test]
fn error() {
    let search_path = "/does/not/exists/a:/etc:/does/not/exists/b";
    let error = rvs_error_new();
    let context = rvs_context_new(CString::new(search_path).unwrap().as_ptr(), 0, error);

    assert_eq!(
        get_error_message(error),
        "Paths not found:\n   \"/does/not/exists/a\"\n   \"/does/not/exists/b\""
    );

    rvs_error_free(error);
    rvs_context_free(context);
}
