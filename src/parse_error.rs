use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub struct IncorrectNumberOfTokensError;

impl fmt::Display for IncorrectNumberOfTokensError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incorrect number of tokens in line")
    }
}
#[derive(Debug)]
pub enum ParseError {
    ParseIntError,
    IncorrectNumberOfTokensError,
}

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        ParseError::ParseIntError
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error")
    }
}
