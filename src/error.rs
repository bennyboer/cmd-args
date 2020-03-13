use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::str::ParseBoolError> for ParserError {
    fn from(e: std::str::ParseBoolError) -> Self {
        ParserError {
            message: format!("Could not parse boolean: {}", e.to_string())
        }
    }
}

impl From<std::num::ParseIntError> for ParserError {
    fn from(e: std::num::ParseIntError) -> Self {
        ParserError {
            message: format!("Could not parse integer: {}", e.to_string())
        }
    }
}

impl From<std::num::ParseFloatError> for ParserError {
    fn from(e: std::num::ParseFloatError) -> Self {
        ParserError {
            message: format!("Could not parse float: {}", e.to_string())
        }
    }
}
