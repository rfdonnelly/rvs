use std::fmt;
use std::io;
use std::ffi::CString;
use libc::c_char;

use rvs;

#[derive(Debug)]
pub struct Error {
    pub message: Option<CString>,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    None,
    Rvs(rvs::Error),
    Io(io::Error),
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error {
            message: None,
            kind,
        }
    }

    pub fn is_err(&self) -> bool {
        match self.kind {
            ErrorKind::None => false,
            ErrorKind::Rvs(_) => true,
            ErrorKind::Io(_) => true,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::None => write!(f, "no error"),
            ErrorKind::Rvs(ref e) => e.fmt(f),
            ErrorKind::Io(ref e) => e.fmt(f),
        }
    }
}

impl From<rvs::Error> for ErrorKind {
    fn from(err: rvs::Error) -> ErrorKind {
        ErrorKind::Rvs(err)
    }
}

#[no_mangle]
pub extern "C" fn rvs_error_new() -> *mut Error {
    Box::into_raw(Box::new(Error::new(ErrorKind::None)))
}

#[no_mangle]
pub extern "C" fn rvs_error_free(err: *mut Error) {
    unsafe {
        Box::from_raw(err);
    }
}

#[no_mangle]
pub extern "C" fn rvs_error_message(err: *mut Error) -> *const c_char {
    let err = unsafe { &mut *err };
    let cmsg = match CString::new(format!("{}", err)) {
        Ok(msg) => msg,
        Err(_) => CString::new("Failed to allocate CString. This shouldn't happen").unwrap(),
    };
    let p = cmsg.as_ptr();
    err.message = Some(cmsg);
    p
}

#[no_mangle]
pub extern "C" fn rvs_error_test(err: *const Error) -> bool {
    let err = unsafe { &*err };

    err.is_err()
}
