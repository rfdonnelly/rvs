use super::*;

use rvs::Context;

use std::ffi::CString;

use error::{
    rvs_error_new,
    rvs_error_free,
    rvs_error_code,
};

fn next_by_name(context: *mut Context, name: &str) -> u32 {
    let handle = rvs_find(context, CString::new(name).unwrap().as_ptr());
    assert!(handle != 0);

    rvs_next(context, handle)
}

mod rvs_seed;
mod rvs_parse;
mod rvs_find;
mod rvs_next;
mod rvs_write_definitions;
