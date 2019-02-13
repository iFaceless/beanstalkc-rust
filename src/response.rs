use crate::proto::Command;
use crate::proto::Status;

#[derive(Debug)]
pub struct Response {
    pub status: Status,
    pub data: String,
}
