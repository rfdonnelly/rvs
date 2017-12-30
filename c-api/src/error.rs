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
    Parse(rvs::ParseError),
    Transform(rvs::TransformError),
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
            ErrorKind::Parse(_) => true,
            ErrorKind::Transform(_) => true,
            ErrorKind::Io(_) => true,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::None => write!(f, "no error"),
            ErrorKind::Parse(ref e) => e.fmt(f),
            ErrorKind::Transform(ref e) => e.fmt(f),
            ErrorKind::Io(ref e) => e.fmt(f),
        }
    }
}

impl From<rvs::Error> for ErrorKind {
    fn from(err: rvs::Error) -> ErrorKind {
        match err {
            rvs::Error::Parse(err) => ErrorKind::Parse(err),
            rvs::Error::Transform(err) => ErrorKind::Transform(err),
            rvs::Error::Io(err) => ErrorKind::Io(err),
        }
    }
}

impl ErrorKind {
    pub fn code(&self) -> u32 {
        match *self {
            ErrorKind::None => 0,
            ErrorKind::Parse(_) => 1,
            ErrorKind::Transform(_) => 2,
            ErrorKind::Io(_) => 3,
        }
    }
}

#[no_mangle]
pub extern fn rvs_error_new() -> *mut Error {
    Box::into_raw(Box::new(Error::new(ErrorKind::None)))
}


#[no_mangle]
pub extern fn rvs_error_free(err: *mut Error) {
    unsafe { Box::from_raw(err); }
}

#[no_mangle]
pub extern fn rvs_error_message(err: *mut Error) -> *const c_char {
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
pub extern fn rvs_error_code(err: *const Error) -> u32 {
    let err = unsafe { &*err };

    err.kind.code()
}
