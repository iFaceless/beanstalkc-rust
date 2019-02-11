use crate::errors::BeanstalkcResult;
use bufstream::BufStream;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::time::Duration;

#[derive(Debug)]
pub struct Beanstalkc {
    host: String,
    port: u32,
    timeout: Option<Duration>,
    stream: Option<BufStream<TcpStream>>,
}

impl Beanstalkc {
    pub fn new() -> Beanstalkc {
        Beanstalkc {
            host: "127.0.0.1".to_string(),
            port: 11300,
            timeout: None,
            stream: None,
        }
    }

    pub fn host(mut self, h: &str) -> Self {
        self.host = h.to_string();
        self
    }

    pub fn port(mut self, p: u32) -> Self {
        self.port = p;
        self
    }

    pub fn timeout(mut self, t: Duration) -> Self {
        self.timeout = Some(t);
        self
    }

    pub fn connect(mut self) -> BeanstalkcResult<Self> {
        let addr: SocketAddr = format!("{}:{}", self.host, self.port).parse()?;
        let tcp_stream = match self.timeout {
            Some(timeout) => TcpStream::connect_timeout(&addr, timeout)?,
            None => TcpStream::connect(&addr)?,
        };
        self.stream = Some(BufStream::new(tcp_stream));
        Ok(self)
    }

    // Producer commands
    pub fn put(&mut self) -> BeanstalkcResult<&mut Self> {
        Ok(self)
    }

    // Worker commands
    pub fn use_(&mut self, tube: &str) -> BeanstalkcResult<&mut Self> {
        Ok(self)
    }

    pub fn quit(&mut self) -> BeanstalkcResult<&mut Self> {
        Ok(self)
    }

    // Other commands
}

impl Drop for Beanstalkc {
    fn drop(&mut self) {
        self.quit();
    }
}
