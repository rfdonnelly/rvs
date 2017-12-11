use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    pub description: String,
}

pub type ParseResult<T> = Result<T, ParseError>;

impl ParseError {
    pub fn new(
        description: String,
    ) -> ParseError {
        ParseError {
            description,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
