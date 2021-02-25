use std::{error::Error, fmt};

use serenity::framework::standard::ArgError;


#[derive(Debug)]
pub enum XKCDError {
    SearchError(String),
    ParseError(String),
    ParseIntError(std::num::ParseIntError),
    ArgParseError(ArgError<std::num::ParseIntError>),
    GetError(reqwest::Error),
    NegativeNumber(i32),
    OutOfRange(u32),
}

impl fmt::Display for XKCDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
impl Error for XKCDError {}

impl From<reqwest::Error> for XKCDError {
    fn from(error: reqwest::Error) -> Self {
        Self::GetError(error)
    }
}

impl From<std::num::ParseIntError> for XKCDError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::ParseIntError(error)
    }
}

impl From<ArgError<std::num::ParseIntError>> for XKCDError {
    fn from(error: ArgError<std::num::ParseIntError>) -> Self {
        Self::ArgParseError(error)
    }
}