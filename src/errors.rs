use std::fmt;
use std::io::Error;
use std::net::AddrParseError;

#[derive(Debug, Clone)]
pub enum BeanstalkcError {
    ConnectionError(String),
    UnexpectedResponse(String),
    CommandFailed(String),
    RequestError(String),
}

impl fmt::Display for BeanstalkcError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            BeanstalkcError::ConnectionError(msg) => format!("Connection error: {}", msg),
            BeanstalkcError::UnexpectedResponse(msg) => format!("Unexpected response: {}", msg),
            BeanstalkcError::CommandFailed(msg) => format!("Command failed: {}", msg),
            BeanstalkcError::RequestError(msg) => format!("Request error: {}", msg),
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

pub type BeanstalkcResult<T> = Result<T, BeanstalkcError>;
