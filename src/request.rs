use std::io::{BufRead, Read, Write};
use std::net::TcpStream;
use std::str::FromStr;

use bufstream::BufStream;

use crate::error::{BeanstalkcError, BeanstalkcResult};
use crate::command::Status;

#[derive(Debug)]
pub struct Response {
    pub status: Status,
    pub params: Vec<String>,
    pub body: Option<String>,
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

#[derive(Debug)]
pub struct Request<'b> {
    stream: &'b mut BufStream<TcpStream>,
}

impl<'b> Request<'b> {
    pub fn new(stream: &'b mut BufStream<TcpStream>) -> Self {
        Request { stream }
    }

    pub fn send(&mut self, message: &[u8]) -> BeanstalkcResult<Response> {
        self.stream.write(message).unwrap();
        self.stream.flush().unwrap();

        let mut line = String::new();
        self.stream.read_line(&mut line);

        if line.trim().is_empty() {
            return Err(BeanstalkcError::UnexpectedResponse(
                "empty response".to_string(),
            ));
        }

        let line_parts: Vec<_> = line.split_whitespace().collect();

        let mut response = Response::default();
        response.status = Status::from_str(line_parts.first().unwrap_or(&""))?;
        response.params = line_parts[1..].iter().map(|&x| x.to_string()).collect();

        let body_byte_count: usize = match response.status {
            Status::Ok => response.params[0].parse()?,
            Status::Reserved => response.params[1].parse()?,
            _ => {
                return Ok(response);
            }
        };

        let mut tmp: Vec<u8> = vec![0; body_byte_count + 2]; // +2 trailing line break
        let body = &mut tmp[..];
        self.stream.read(body)?;
        tmp.truncate(body_byte_count);
        response.body = Some(String::from_utf8(tmp)?);

        Ok(response)
    }
}
