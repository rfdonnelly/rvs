use super::*;

use rvs;

use std::ffi::CString;
use std::ffi::CStr;

use error::{rvs_error_free, rvs_error_new, rvs_error_test};

fn get_error_message(error: *mut Error) -> String {
    let c_buf = rvs_error_message(error);
    let c_str = unsafe { CStr::from_ptr(c_buf) };
    let error_message = c_str.to_str().unwrap();

    error_message.to_owned()
}

fn assert_starts_with<S0, S1>(s: S0, start: S1)
where
    S0: AsRef<str>,
    S1: AsRef<str>,
{
    if !s.as_ref().starts_with(start.as_ref()) {
        panic!("'{}' doesn't start with '{}'", s.as_ref(), start.as_ref());
    }
}

fn next_by_name(model: *mut rvs::Model, name: &str) -> u32 {
    let handle = rvs_get(model, CString::new(name).unwrap().as_ptr());
    assert!(handle != 0);

    rvs_next(model, handle)
}

mod rvs_context_new;
mod rvs_seed;
mod rvs_parse;
mod rvs_get;
mod rvs_next;
mod rvs_write_definitions;
