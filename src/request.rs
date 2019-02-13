use std::io::{BufRead, Read, Write};
use std::net::TcpStream;

use bufstream::BufStream;

use crate::error::BeanstalkcResult;
use crate::proto::Status;
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
        self.stream.write(message).unwrap();
        self.stream.flush().unwrap();

        let mut resp = String::new();
        self.stream.read_line(&mut resp);
        println!("{}", resp);

        Ok(Response {
            data: resp,
            status: Status::Ok,
        })
    }
}
