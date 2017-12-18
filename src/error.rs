use std::fmt;

#[derive(Debug)]
pub struct TransformError {
    pub description: String,
}

pub type TransformResult<T> = Result<T, TransformError>;

impl TransformError {
    pub fn new(
        description: String,
    ) -> TransformError {
        TransformError {
            description,
        }
    }
}

impl fmt::Display for TransformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
