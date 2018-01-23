use super::*;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::ffi::CStr;

use tempdir::TempDir;

fn assert_contents_eq(actual: &Path, expected: &Path) {
    let mut actual_file = File::open(actual).expect(&format!("Cannot open {:?}", actual));
    let mut expected_file = File::open(expected).expect(&format!("Cannot open {:?}", expected));

    let mut actual_contents = String::new();
    let mut expected_contents = String::new();

    actual_file
        .read_to_string(&mut actual_contents)
        .expect(&format!("Cannot open {:?}", actual));
    expected_file
        .read_to_string(&mut expected_contents)
        .expect(&format!("Cannot open {:?}", expected));

    assert_diff!(&actual_contents, &expected_contents, " ", 0);
}

fn parse_write(input: &Path, output: &Path) {
    let error = rvs_error_new();
    let context = rvs_context_new(CString::new("").unwrap().as_ptr(), 0, error);
    assert!(!rvs_error_test(error));

    println!("Parsing {:?}", input);
    rvs_parse(
        context,
        CString::new(input.to_string_lossy().as_bytes())
            .unwrap()
            .as_ptr(),
        error,
    );
    report_error("rvs_parse", error);
    assert!(!rvs_error_test(error));

    let model = rvs_model_new();
    rvs_transform(context, model, error);
    report_error("rvs_transform", error);
    assert!(!rvs_error_test(error));

    println!("Writing {:?}", output);
    rvs_write_definitions(
        model,
        CString::new(output.to_string_lossy().as_bytes())
            .unwrap()
            .as_ptr(),
        error,
    );
    report_error("rvs_write_definitions", error);
    assert!(!rvs_error_test(error));

    rvs_error_free(error);
    rvs_model_free(model);
}

fn report_error(call: &str, error: *mut Error) {
    if rvs_error_test(error) {
        let c_buf = rvs_error_message(error);
        let c_str = unsafe { CStr::from_ptr(c_buf) };
        let error_message = c_str.to_str().unwrap();
        println!("{} error: {}", call, error_message);
    }
}

#[test]
fn basic() {
    let files = vec!["readme.rvs"];
    let input_path = Path::new("../examples");
    let output0_path = TempDir::new("rvs-output0").unwrap();
    let output0_path = output0_path.path();
    let output1_path = TempDir::new("rvs-output1").unwrap();
    let output1_path = output1_path.path();
    let expected_path = Path::new("tests/rvs_write_definitions");

    for file in files {
        let input = input_path.join(file);
        let output0 = output0_path.join(file);
        let output1 = output1_path.join(file);
        let expected = expected_path.join(file);

        parse_write(&input, &output0);
        assert_contents_eq(&output0, &expected);

        parse_write(&output0, &output1);
        assert_contents_eq(&output1, &expected);
    }
}
