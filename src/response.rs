use crate::command::Status;
use crate::error::{BeanstalkcError, BeanstalkcResult};
use serde_yaml;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    pub status: Status,
    pub params: Vec<String>,
    pub body: Option<Vec<u8>>,
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

    pub fn body_as_map(&self) -> BeanstalkcResult<HashMap<String, String>> {
        let res = match &self.body {
            None => HashMap::default(),
            Some(b) => {
                let b = std::str::from_utf8(b)?;
                serde_yaml::from_str(b).unwrap_or_default()
            },
        };
        Ok(res)
    }

    pub fn body_as_vec(&self) -> BeanstalkcResult<Vec<String>> {
        let res = match &self.body {
            None => Vec::default(),
            Some(b) => {
                let b = std::str::from_utf8(b)?;
                serde_yaml::from_str(b).unwrap_or_default()
            }
        };
        Ok(res)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_job_id() {
        let resp = Response {
            status: Status::Inserted,
            params: vec!["100".to_string()],
            body: None,
        };
        let r = resp.job_id();
        assert!(r.is_ok());
        assert_eq!(100, r.unwrap());
    }

    #[test]
    fn test_get_int_param() {
        let resp = Response {
            status: Status::Reserved,
            params: vec!["100".to_string(), "5".to_string()],
            body: Some(b"hello".to_vec()),
        };

        let r = resp.get_int_param(1);
        assert!(r.is_ok());
        assert_eq!(5, r.unwrap());
    }

    #[test]
    fn test_get_body_as_vec() {
        let resp = Response {
            status: Status::Reserved,
            params: vec![],
            body: Some(b"- default\n- jobs\n".to_vec()),
        };

        let tubes = resp.body_as_vec().unwrap();
        assert_eq!(vec!["default".to_string(), "jobs".to_string()], tubes);
    }

    #[test]
    fn test_get_body_as_map() {
        let resp = Response {
            status: Status::Ok,
            params: vec![],
            body: Some(b"name: default\nuptime: 12345".to_vec()),
        };

        let stats = resp.body_as_map().unwrap();
        assert_eq!(stats["name"], "default".to_string());
        assert_eq!(stats["uptime"], "12345".to_string());
    }
}
