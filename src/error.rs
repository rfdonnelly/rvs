use std::fmt;
use std::io;
use std::error;

use rvs_parser;
use rvs_parser::ParseError;

#[derive(Debug)]
pub enum Error {
    Parse(ParseError),
    Transform(TransformError),
    Io(io::Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub struct TransformError {
    pub description: String,
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Parse(ref err) => Some(err),
            Error::Transform(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Parse(ref err) => err.fmt(f),
            Error::Transform(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
        }
    }
}

impl From<rvs_parser::Error> for Error {
    fn from(err: rvs_parser::Error) -> Error {
        match err {
            rvs_parser::Error::Io(err) => Error::Io(err),
            rvs_parser::Error::Parse(err) => Error::Parse(err),
        }
    }
}

impl From<TransformError> for Error {
    fn from(err: TransformError) -> Error {
        Error::Transform(err)
    }
}

pub type TransformResult<T> = ::std::result::Result<T, TransformError>;

impl TransformError {
    pub fn new(description: String) -> TransformError {
        TransformError { description }
    }
}

impl error::Error for TransformError {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl fmt::Display for TransformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
