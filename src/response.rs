use crate::command::Status;
use crate::error::{BeanstalkcError, BeanstalkcResult};
use serde_yaml;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    pub status: Status,
    pub params: Vec<String>,
    pub body: Option<String>,
}

impl Response {
    pub fn job_id(&self) -> BeanstalkcResult<u64> {
        self.get_int_param(0)
    }

    pub fn get_int_param(&self, index: usize) -> BeanstalkcResult<u64> {
        let value: u64 = self.get_param(index)?.parse()?;
        Ok(value)
    }

    pub fn get_param(&self, index: usize) -> BeanstalkcResult<String> {
        match self.params.get(index) {
            Some(x) => Ok(x.to_string()),
            None => Err(BeanstalkcError::UnexpectedResponse(format!(
                "param not found: {}",
                index
            ))),
        }
    }

    pub fn body_as_map(&self) -> HashMap<String, String> {
        match &self.body {
            None => HashMap::default(),
            Some(b) => serde_yaml::from_str(b.as_str()).unwrap_or_default(),
        }
    }

    pub fn body_as_vec(&self) -> Vec<String> {
        match &self.body {
            None => Vec::default(),
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
