use crate::command::Status;
use serde_yaml;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    pub status: Status,
    pub params: Vec<u64>,
    pub body: Option<String>,
}

impl Response {
    pub fn body_as_map(&self) -> HashMap<String, String> {
        match &self.body {
            None => HashMap::default(),
            Some(b) => serde_yaml::from_str(b.as_str()).unwrap_or_default(),
        }
    }
}

impl Default for Response {
    fn default() -> Self {
        Response {
            status: Status::Ok,
            params: vec![],
            body: None,
        }
    }
}
