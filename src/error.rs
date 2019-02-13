use std::fmt;
use std::io::Error;
use std::net::AddrParseError;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

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

impl From<Error> for BeanstalkcError {
    fn from(err: Error) -> Self {
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

pub type BeanstalkcResult<T> = Result<T, BeanstalkcError>;
