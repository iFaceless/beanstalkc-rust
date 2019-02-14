use std::io::{BufRead, Read, Write};
use std::net::TcpStream;
use std::str::FromStr;

use bufstream::BufStream;

use crate::command::Status;
use crate::error::{BeanstalkcError, BeanstalkcResult};
use crate::response::Response;

#[derive(Debug)]
pub struct Request<'b> {
    stream: &'b mut BufStream<TcpStream>,
}

impl<'b> Request<'b> {
    pub fn new(stream: &'b mut BufStream<TcpStream>) -> Self {
        Request { stream }
    }

    pub fn send(&mut self, message: &[u8]) -> BeanstalkcResult<Response> {
        let _ = self.stream.write(message)?;
        self.stream.flush()?;

        let mut line = String::new();
        self.stream.read_line(&mut line)?;

        if line.trim().is_empty() {
            return Err(BeanstalkcError::UnexpectedResponse(
                "empty response".to_string(),
            ));
        }

        let line_parts: Vec<_> = line.split_whitespace().collect();

        let mut response = Response::default();
        response.status = Status::from_str(line_parts.first().unwrap_or(&""))?;
        response.params = line_parts[1..].iter().map(|&x| x.to_string()).collect();

        let body_byte_count = match response.status {
            Status::Ok => response.get_int_param(0)?,
            Status::Reserved => response.get_int_param(1)?,
            Status::Found => response.get_int_param(1)?,
            _ => {
                return Ok(response);
            }
        } as usize;

        let mut tmp: Vec<u8> = vec![0; body_byte_count + 2]; // +2 trailing line break
        let body = &mut tmp[..];
        self.stream.read_exact(body)?;
        tmp.truncate(body_byte_count);
        response.body = Some(String::from_utf8(tmp)?);

        Ok(response)
    }
}
