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

use rvs::Context;
use rvs::Seed;
use rvs;

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

/// Sets the search path used for `import`
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

    if let Err(e) = context.search_path.set(&r_str) {
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
            let is_file = !entry.contains("=") && !entry.contains("import");

            let parser_string =
                if is_file {
                    let path = Path::new(&entry);

                    let mut file = match File::open(&path) {
                        Err(e) => {
                            if !error.is_null() {
                                unsafe {
                                    *error = Error::new(ErrorKind::Io(e));
                                }
                            }

                            return;
                        }
                        Ok(file) => file,
                    };

                    let mut contents = String::new();
                    if let Err(e) = file.read_to_string(&mut contents) {
                        if !error.is_null() {
                            unsafe {
                                *error = Error::new(ErrorKind::Io(e));
                            }
                        }

                        return;
                    };

                    contents
                } else {
                    entry.to_owned() + ";"
                };

            if let Err(e) = rvs::parse(&parser_string, &mut context) {
                if !error.is_null() {
                    unsafe {
                        *error = Error::new(From::from(e))
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub extern fn rvs_transform(
    context: *mut Context,
    error: *mut Error
) {
    let mut context = unsafe { &mut *context };

    if let Err(e) = rvs::transform(&mut context) {
        if !error.is_null() {
            unsafe {
                *error = Error::new(From::from(e))
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
        SequenceHandle::from(index).to_raw()
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

#[no_mangle]
pub extern fn rvs_write_definitions(
    context: *const Context,
    s: *const c_char,
    error: *mut Error
) {
    let c_str = unsafe { CStr::from_ptr(s) };
    let r_str = c_str.to_str().unwrap();
    let context = unsafe { &*context };

    let path = Path::new(r_str);

    let mut file = match File::create(&path) {
        Err(e) => {
            if !error.is_null() {
                unsafe {
                    *error = Error::new(ErrorKind::Io(e));
                }
            }

            return;
        },
        Ok(file) => file,
    };

    let variables = format!("{}", context.variables);
    if let Err(e) = file.write_all(variables.as_bytes()) {
        if !error.is_null() {
            unsafe {
                *error = Error::new(ErrorKind::Io(e));
            }

            return;
        }
    }
}
