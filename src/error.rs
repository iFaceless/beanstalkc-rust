use std::error::Error;
use std::fmt;
use std::io;
use std::net::AddrParseError;
use std::num::ParseIntError;
use std::string::FromUtf8Error;
use std::str::Utf8Error;

#[derive(Debug, Clone)]
pub enum BeanstalkcError {
    ConnectionError(String),
    UnexpectedResponse(String),
    CommandFailed(String),
}

impl fmt::Display for BeanstalkcError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            BeanstalkcError::ConnectionError(msg) => format!("Connection error: {}", msg),
            BeanstalkcError::UnexpectedResponse(msg) => format!("Unexpected response: {}", msg),
            BeanstalkcError::CommandFailed(msg) => format!("Command failed: {}", msg),
        };

        write!(formatter, "{}", description)
    }
}

impl Error for BeanstalkcError {}

impl From<io::Error> for BeanstalkcError {
    fn from(err: io::Error) -> Self {
        BeanstalkcError::ConnectionError(err.to_string())
    }
}

impl From<AddrParseError> for BeanstalkcError {
    fn from(err: AddrParseError) -> Self {
        BeanstalkcError::ConnectionError(err.to_string())
    }
}

impl From<ParseIntError> for BeanstalkcError {
    fn from(err: ParseIntError) -> Self {
        BeanstalkcError::UnexpectedResponse(err.to_string())
    }
}

impl From<FromUtf8Error> for BeanstalkcError {
    fn from(err: FromUtf8Error) -> Self {
        BeanstalkcError::UnexpectedResponse(err.to_string())
    }
}

impl From<Utf8Error> for BeanstalkcError {
    fn from(err: Utf8Error) -> Self {
        BeanstalkcError::UnexpectedResponse(err.to_string())
    }
}

pub type BeanstalkcResult<T> = Result<T, BeanstalkcError>;
