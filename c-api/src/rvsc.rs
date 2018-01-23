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

use rvs;

use context::Context;
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

/// Allocates and returns a new Context
///
/// The pointer returned is owned by the caller and is freed by a call to `rvs_transform` or
/// `rvs_context_free`.
///
/// # Arguments
///
/// * search_path - A colon separated list of paths to search for `import`s.
/// * seed - The initial seed for all variable PRNGs.
///
/// # Errors
///
/// Error will be reported for search paths that do not exist.  If the search path string contains
/// a mix of paths that do and do not exist, the paths that do exist will be added to the internal
/// search path.
///
/// A valid Context pointer will be returned and will need to be freed by the caller regardless of
/// error or no error.
#[no_mangle]
pub extern "C" fn rvs_context_new(
    search_path: *const c_char,
    seed: u32,
    error: *mut Error,
) -> *mut Context {
    let c_str = unsafe { CStr::from_ptr(search_path) };
    let r_str = c_str.to_str().unwrap();

    let search_path = match rvs::SearchPath::from_string(&r_str) {
        Ok(search_path) => search_path,
        Err(e) => {
            if !error.is_null() {
                unsafe { *error = Error::new(ErrorKind::Io(e)) }
            }

            Default::default()
        }
    };

    let seed = rvs::Seed::from_u32(seed);

    Box::into_raw(Box::new(Context::new(search_path, seed)))
}

/// Parses a semicolon delimited string of Rvs statements and/or Rvs files.
///
/// A terminating semicolon is optional.
///
/// # Errors
///
/// Errors are reported via the optional error struct pointer if available.  The following errors
/// types are possible:
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
pub extern "C" fn rvs_parse(context: *mut Context, s: *const c_char, error: *mut Error) {
    assert!(!context.is_null());
    assert!(!s.is_null());

    let c_str = unsafe { CStr::from_ptr(s) };
    let r_str = c_str.to_str().unwrap();
    let context = unsafe { &mut *context };

    for entry in r_str.split(';') {
        if !entry.is_empty() {
            let is_file = !entry.contains("=") && !entry.contains("import");

            let parser_string = if is_file {
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

            if let Err(e) = context.parse(&parser_string) {
                if !error.is_null() {
                    unsafe { *error = Error::new(From::from(e)) }
                }
            }
        }
    }
}

/// Creates a new Model
///
/// The pointer returned is owned by the caller and is freed by a call to `rvs_model_free`.
#[no_mangle]
pub extern "C" fn rvs_model_new() -> *mut rvs::Model {
    Box::into_raw(Box::new(rvs::Model::new()))
}

/// Transforms an AST into an object model
///
/// # Arguments
///
/// * context - (required) A Context pointer.  Created by `rvs_context_new`.  Freed by `rvs_transform`.
/// * model - (required) A Model pointer.  Created by `rvs_model_new`.  Freed by `rvs_model_free`
/// * error - (optional) An Error pointer.  Used to report any errors that may occur.
///
/// # Errors
///
/// Errors are reported via the optional error struct pointer if available.  The following errors
/// types are possible:
///
/// * Transform errors
#[no_mangle]
pub extern "C" fn rvs_transform(context: *mut Context, model: *mut rvs::Model, error: *mut Error) {
    assert!(!context.is_null());
    assert!(!model.is_null());

    let context_deref = unsafe { &mut *context };
    let mut model = unsafe { &mut *model };

    if let Err(e) = context_deref.transform(&mut model) {
        if !error.is_null() {
            unsafe { *error = Error::new(From::from(e)) }
        }
    }

    unsafe { Box::from_raw(context) };
}

/// Frees a Context previously allocated by `rvs_context_new`
///
/// This is for error scenarios only.  In a non-error scenario, `rvs_transform` is used to free the
/// Context.
#[no_mangle]
pub extern "C" fn rvs_context_free(context: *mut Context) {
    assert!(!context.is_null());
    unsafe {
        Box::from_raw(context);
    }
}

/// Frees a Model previously allocated by `rvs_transform`
#[no_mangle]
pub extern "C" fn rvs_model_free(model: *mut rvs::Model) {
    assert!(!model.is_null());
    unsafe {
        Box::from_raw(model);
    }
}

/// Returns the handle of a variable
///
/// The callee owns the handle.  The handle is valid until `rvs_model_free()` is called.
///
/// # Errors
///
/// * Returns 0 if variable does not exist
///
/// # Panics
///
/// * If any pointer arguments are null
#[no_mangle]
pub extern "C" fn rvs_get(model: *mut rvs::Model, name: *const c_char) -> SequenceHandleRaw {
    assert!(!model.is_null());
    assert!(!name.is_null());

    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name_rstr = name_cstr.to_str().unwrap();

    let model = unsafe { &mut *model };
    if let Some(index) = model.get_variable_index(name_rstr) {
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
pub extern "C" fn rvs_next(model: *mut rvs::Model, handle: SequenceHandleRaw) -> u32 {
    assert!(!model.is_null());

    let model = unsafe { &mut *model };
    let handle = SequenceHandle(handle);
    match model.get_variable_by_index(handle.into()) {
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
pub extern "C" fn rvs_prev(model: *mut rvs::Model, handle: SequenceHandleRaw) -> u32 {
    assert!(!model.is_null());

    let model = unsafe { &mut *model };
    let handle = SequenceHandle(handle);

    match model.get_variable_by_index(handle.into()) {
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
pub extern "C" fn rvs_done(model: *mut rvs::Model, handle: SequenceHandleRaw) -> bool {
    assert!(!model.is_null());

    let model = unsafe { &mut *model };
    let handle = SequenceHandle(handle);

    match model.get_variable_by_index(handle.into()) {
        Some(variable) => variable.borrow().done(),
        None => false,
    }
}

#[no_mangle]
pub extern "C" fn rvs_write_definitions(
    model: *const rvs::Model,
    s: *const c_char,
    error: *mut Error,
) {
    assert!(!model.is_null());
    assert!(!s.is_null());

    let c_str = unsafe { CStr::from_ptr(s) };
    let r_str = c_str.to_str().unwrap();
    let model = unsafe { &*model };

    let path = Path::new(r_str);

    let mut file = match File::create(&path) {
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

    let variables = format!("{}", model);
    if let Err(e) = file.write_all(variables.as_bytes()) {
        if !error.is_null() {
            unsafe {
                *error = Error::new(ErrorKind::Io(e));
            }

            return;
        }
    }
}
