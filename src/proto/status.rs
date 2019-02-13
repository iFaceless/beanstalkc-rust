use crate::error::BeanstalkcError;
use std::str::FromStr;

#[derive(Debug)]
pub enum Status {
    Ok,
    Found,
    NotFound,
    Reserved,
    DeadlineSoon,
    TimedOut,
    Deleted,
    Released,
    Buried,
    Kicked,
    Touched,
    Inserted,
    NotIgnored,
    OutOfMemory,
    InternalError,
    Draining,
    BadFormat,
    UnknownCommand,
    ExpectedCRLF,
    JobTooBig,
}

impl FromStr for Status {
    type Err = BeanstalkcError;

    fn from_str(s: &str) -> Result<Self, BeanstalkcError> {
        let s = match s {
            "OK" => Status::Ok,
            "FOUND" => Status::Found,
            "NOT_FOUND" => Status::NotFound,
            "RESERVED" => Status::Reserved,
            "DEADLINE_SOON" => Status::DeadlineSoon,
            "TIMED_OUT" => Status::TimedOut,
            "DELETED" => Status::Deleted,
            "RELEASED" => Status::Released,
            "BURIED" => Status::Buried,
            "KICKED" => Status::Kicked,
            "TOUCHED" => Status::Touched,
            "INSERTED" => Status::Inserted,
            "NOT_IGNORED" => Status::NotIgnored,
            "OUT_OF_MEMORY" => Status::OutOfMemory,
            "INTERNAL_ERROR" => Status::InternalError,
            "DRAINING" => Status::Draining,
            "BAD_FORMAT" => Status::BadFormat,
            "UNKNOWN_COMMAND" => Status::UnknownCommand,
            "EXPECTED_CRLF" => Status::ExpectedCRLF,
            "JOB_TOO_BIG" => Status::JobTooBig,
            _ => {
                return Err(BeanstalkcError::CommandFailed(s.to_string()));
            }
        };
        Ok(s)
    }
}
