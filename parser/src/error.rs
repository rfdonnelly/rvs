use std::fmt;
use std::io;
use std::error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Parse(ParseError),
    Io(io::Error),
}

#[derive(Debug)]
pub struct ParseError {
    pub description: String,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Parse(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Parse(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Parse(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error::Parse(err)
    }
}

pub type ParseResult<T> = ::std::result::Result<T, ParseError>;

impl ParseError {
    pub fn new(description: String) -> ParseError {
        ParseError { description }
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
