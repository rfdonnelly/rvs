use super::*;

use rvs;

use std::ffi::CString;

use error::{
    rvs_error_new,
    rvs_error_free,
    rvs_error_test,
};

fn next_by_name(model: *mut rvs::Model, name: &str) -> u32 {
    let handle = rvs_get(model, CString::new(name).unwrap().as_ptr());
    assert!(handle != 0);

    rvs_next(model, handle)
}

mod rvs_seed;
mod rvs_parse;
mod rvs_get;
mod rvs_next;
mod rvs_write_definitions;
