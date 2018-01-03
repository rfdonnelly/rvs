use super::*;

use std::path::Path;
use std::fs::File;
use std::io::Read;

use tempdir::TempDir;

fn assert_contents_eq(actual: &Path, expected: &Path) {
    let mut actual_file = File::open(actual).expect(&format!("Cannot open {:?}", actual));
    let mut expected_file = File::open(expected).expect(&format!("Cannot open {:?}", expected));

    let mut actual_contents = String::new();
    let mut expected_contents = String::new();

    actual_file.read_to_string(&mut actual_contents).expect(&format!("Cannot open {:?}", actual));
    expected_file.read_to_string(&mut expected_contents).expect(&format!("Cannot open {:?}", expected));

    assert_diff!(&actual_contents, &expected_contents, " ", 0);
}

fn parse_write(input: &Path, output: &Path) {
    let context = rvs_context_new();
    let error = rvs_error_new();

    rvs_parse(context, CString::new(input.to_string_lossy().as_bytes()).unwrap().as_ptr(), error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    rvs_transform(context, error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    rvs_write_definitions(context, CString::new(output.to_string_lossy().as_bytes()).unwrap().as_ptr(), error);
    assert_eq!(rvs_error_code(error), ErrorKind::None.code());

    rvs_error_free(error);
    rvs_context_free(context);
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
