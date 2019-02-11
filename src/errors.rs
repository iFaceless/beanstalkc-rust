use std::fmt;
use std::io::Error;
use std::net::AddrParseError;

#[derive(Debug, Clone)]
pub enum BeanstalkcError {
    ConnectionError(String),
    RequestError,
}

impl fmt::Display for BeanstalkcError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            BeanstalkcError::ConnectionError(err_msg) => format!("Connection error: {}", err_msg),
            BeanstalkcError::RequestError => "Request error occurred".to_string(),
        };

        write!(formatter, "{}", description)
    }
}

impl From<Error> for BeanstalkcError {
    fn from(err: Error) -> Self {
        BeanstalkcError::ConnectionError(format!("{}", err))
    }
}

impl From<AddrParseError> for BeanstalkcError {
    fn from(err: AddrParseError) -> Self {
        BeanstalkcError::ConnectionError(format!("{}", err))
    }
}

pub type BeanstalkcResult<T> = Result<T, BeanstalkcError>;
