use super::*;

#[test]
fn error() {
    #[cfg(windows)]
    let search_path = r"c:\does\not\exists\a;c:\;c:\does\not\exists\b";
    #[cfg(not(windows))]
    let search_path = "/does/not/exists/a:/etc:/does/not/exists/b";

    let error = rvs_error_new();
    let context = rvs_context_new(CString::new(search_path).unwrap().as_ptr(), 0, error);

    #[cfg(windows)]
    let expected = "Paths not found:\n   \"c:\\does\\not\\exists\\a\"\n   \"c:\\does\\not\\exists\\b\"";
    #[cfg(not(windows))]
    let expected = "Paths not found:\n   \"/does/not/exists/a\"\n   \"/does/not/exists/b\"";


    assert_eq!(
        get_error_message(error),
        expected
    );

    rvs_error_free(error);
    rvs_context_free(context);
}
