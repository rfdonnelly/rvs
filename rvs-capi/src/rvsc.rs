//! Rvs C API
//!
//! Provides a C API for parsing and evaluating random variables.

use std::panic::catch_unwind;
use libc::uint32_t;
use libc::c_char;
use std::ffi::CStr;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use rvs::types::Context;
use rvs::types::Seed;
use rvs::parse;

use error::Error;
use error::ErrorKind;

type SequenceHandleRaw = uint32_t;
struct SequenceHandle(SequenceHandleRaw);

impl SequenceHandle {
    pub fn to_raw(self) -> SequenceHandleRaw {
        self.0
    }
}

impl Into<usize> for SequenceHandle {
    fn into(self) -> usize {
        (self.to_raw() - 1) as usize
    }
}

impl From<usize> for SequenceHandle {
    fn from(index: usize) -> SequenceHandle {
        SequenceHandle((index + 1) as uint32_t)
    }
}

/// Allocates and returns a new context.
///
/// The caller owns the context and must call `rvs_context_free()` to free the context.
#[no_mangle]
pub extern fn rvs_context_new() -> *mut Context {
    Box::into_raw(Box::new(
        Context::new()
    ))
}

/// Frees a context.
#[no_mangle]
pub extern fn rvs_context_free(context: *mut Context) {
    if context.is_null() { return }
    unsafe { Box::from_raw(context); }
}

/// Sets the search path used for `require`
///
/// The string must be a colon separated list of paths.
///
/// # Errors
///
/// Error will be reported for parsed paths that do not exist.  If the search path string contains
/// a mix of paths that do and do not exist, the paths that do exist will be added to the internal
/// search path.
///
/// # Panics
///
/// If any pointer arguments are null.
#[no_mangle]
pub extern fn rvs_search_path(
    context: *mut Context,
    path: *const c_char,
    error: *mut Error
) {
    assert!(!context.is_null());

    let context = unsafe { &mut *context };
    let c_str = unsafe { CStr::from_ptr(path) };
    let r_str = c_str.to_str().unwrap();

    if let Err(e) = context.requires.set_search_path(&r_str) {
        if !error.is_null() {
            unsafe {
                *error = Error::new(ErrorKind::Io(e))
            }
        }
    }
}


/// Sets the seed for all future calls to `rvs_parse()`.
///
/// Should be called before `rvs_parse()`.
#[no_mangle]
pub extern fn rvs_seed(context: *mut Context, seed: u32) {
    assert!(!context.is_null());

    let context = unsafe { &mut *context };
    context.seed = Seed::from_u32(seed);
}

/// Parses a semicolon delimited string of Rvs statements and/or Rvs files.
///
/// A terminating semicolon is optional.
///
/// # Errors
///
/// Errors will be reported via the optional error struct pointer if available.  The following
/// errors types are possible:
///
/// * Parsing errors
/// * IO errors
///
/// # Panics
///
/// If any pointer arguments are null.
///
/// # Examples
///
/// A single Rvs statement:
///
/// "a = 5"
///
/// A single Rvs file:
///
/// "example.rvs"
///
/// An Rvs file and an Rvs statement:
///
/// "example.rvs; a = 5;"
#[no_mangle]
pub extern fn rvs_parse(
    context: *mut Context,
    s: *const c_char,
    error: *mut Error
) {
    assert!(!context.is_null());
    assert!(!s.is_null());

    let c_str = unsafe { CStr::from_ptr(s) };
    let r_str = c_str.to_str().unwrap();

    let mut context = unsafe { &mut *context };

    for entry in r_str.split(';') {
        if !entry.is_empty() {
            let is_file = !entry.contains("=") && !entry.contains("require");

            let parser_string =
                if is_file {
                    let path = Path::new(&entry);
                    if !path.exists() {
                        panic!("path does not exist: {}", path.display());
                    }

                    let mut file = match File::open(&path) {
                        Err(e) => panic!("could not open {}: {}", path.display(), ::std::error::Error::description(&e)),
                        Ok(file) => file,
                    };

                    let mut contents = String::new();
                    match file.read_to_string(&mut contents) {
                        Err(e) => panic!("could not read {}: {}", path.display(), ::std::error::Error::description(&e)),
                        Ok(_) => (),
                    };

                    contents
                } else {
                    entry.to_owned() + ";"
                };

            if let Err(e) = parse(&parser_string, &mut context) {
                if !error.is_null() {
                    unsafe {
                        *error = Error::new(ErrorKind::Parse(e))
                    }
                }
            }
        }
    }
}

/// Returns the handle of a variable
///
/// The callee owns the handle.  The handle is valid until `rvs_context_free()` is called.
///
/// # Errors
///
/// * Returns 0 if variable does not exist
///
/// # Panics
///
/// * If any pointer arguments are null
#[no_mangle]
pub extern fn rvs_find(context: *mut Context, name: *const c_char) -> SequenceHandleRaw {
    assert!(!context.is_null());
    assert!(!name.is_null());

    let id_cstr = unsafe { CStr::from_ptr(name) };
    let id_rstr = id_cstr.to_str().unwrap();

    let context = unsafe { &mut *context };
    if let Some(index) = context.variables.get_index(id_rstr) {
        SequenceHandle::from(*index).to_raw()
    } else {
        0
    }
}

/// Returns the next value of a variable via the result pointer
///
/// # Errors
///
/// Returns 0 if handle is invalid.
///
/// # Panics
///
/// * If any pointer arguments are null
/// * If handle doesn't exist
#[no_mangle]
pub extern fn rvs_next(context: *mut Context, handle: SequenceHandleRaw) -> u32 {
    assert!(!context.is_null());

    let context = unsafe { &mut *context };
    let handle = SequenceHandle(handle);
    match context.variables.get_by_index(handle.into()) {
        Some(variable) => variable.borrow_mut().next(),
        None => 0,
    }
}

/// Returns the previous value of a variable
///
/// # Errors
///
/// * Returns 0 if handle is invalid
/// * Returns 0 if `rvs_next` has not been called
///
/// # Panics
///
/// * If any pointer arguments are null
/// * If handle doesn't exist
#[no_mangle]
pub extern fn rvs_prev(context: *mut Context, handle: SequenceHandleRaw) -> u32 {
    assert!(!context.is_null());

    let context = unsafe { &mut *context };
    let handle = SequenceHandle(handle);

    match context.variables.get_by_index(handle.into()) {
        Some(variable) => variable.borrow().prev(),
        None => 0,
    }
}

/// Returns the done value of a variable via the result pointer
///
/// # Errors
///
/// * Returns false if handle is invalid
/// * Returns false if `rvs_next` has not been called
///
/// # Panics
///
/// * If any pointer arguments are null
/// * If handle doesn't exist
#[no_mangle]
pub extern fn rvs_done(context: *mut Context, handle: SequenceHandleRaw) -> bool {
    assert!(!context.is_null());

    let context = unsafe { &mut *context };
    let handle = SequenceHandle(handle);

    match context.variables.get_by_index(handle.into()) {
        Some(variable) => variable.borrow().done(),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    mod rvs_seed {
        use super::*;

        fn next(seed: u32, s: &str) -> u32 {
            let context = rvs_context_new();
            let error = rvs_error_new();
            rvs_seed(context, seed);

            let s = format!("a = {};", s);
            rvs_parse(context, CString::new(s).unwrap().as_ptr(), error);
            assert_eq!(rvs_error_code(error), ErrorKind::None.code());

            let handle = rvs_find(context, CString::new("a").unwrap().as_ptr());
            assert!(handle != 0);

            let value = rvs_next(context, handle);

            rvs_error_free(error);
            rvs_context_free(context);

            value
        }

        #[test]
        fn basic() {
            let s = "[0, 0xffff_ffff]";

            let seed0_value0 = next(0, s);
            let seed1_value0 = next(1, s);
            let seed0_value1 = next(0, s);
            let seed1_value1 = next(1, s);

            assert!(seed0_value0 != seed1_value0);
            assert_eq!(seed0_value0, seed0_value1);
            assert_eq!(seed1_value0, seed1_value1);
        }
    }

    mod rvs_parse {
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
        fn require() {
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

            rvs_parse(context, CString::new("require '../examples/require.rvs'").unwrap().as_ptr(), error);
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

            let variable = unsafe { (*context).variables.get("a").unwrap() };
            let value = variable.borrow_mut().next();
            assert!(value == 0 || value == 1);

            rvs_error_free(error);
            rvs_context_free(context);
        }

        #[test]
        fn parse_error() {
            // use rvs::grammar::ParseError;

            let context = rvs_context_new();
            let error = rvs_error_new();

            rvs_parse(context, CString::new("a = 1;\n1 = b;").unwrap().as_ptr(), error);
            // FIXME: Check error message
            // println!("{}", unsafe { *error });
            // assert_eq!(rvs_error_code(error), ErrorKind::Parse(ParseError::new()).code());
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

            let handle = rvs_find(context, CString::new("a").unwrap().as_ptr());
            assert!(handle != 0);

            let value = rvs_next(context, handle);
            assert_eq!(value, 2);

            rvs_error_free(error);
            rvs_context_free(context);
        }
    }

    mod rvs_find {
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

            let handle = rvs_find(context, CString::new("a").unwrap().as_ptr());
            assert!(handle != 0);

            rvs_error_free(error);
            rvs_context_free(context);
        }
    }

    mod rvs_next {
        use super::*;

        #[test]
        fn found() {
            let context = rvs_context_new();
            let error = rvs_error_new();

            rvs_parse(context, CString::new("a=5;").unwrap().as_ptr(), error);
            assert_eq!(rvs_error_code(error), ErrorKind::None.code());

            let handle = rvs_find(context, CString::new("a").unwrap().as_ptr());
            assert!(handle != 0);

            let value = rvs_next(context, handle);
            assert_eq!(value, 5);

            rvs_error_free(error);
            rvs_context_free(context);
        }

        #[test]
        fn not_found() {
            let context = rvs_context_new();

            let handle = 1;
            let value = rvs_next(context, handle);
            assert_eq!(value, 0);

            rvs_context_free(context);
        }
    }
}
