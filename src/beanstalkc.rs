use std::collections::HashMap;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::time::Duration;

use bufstream::BufStream;

use crate::command;
use crate::config::*;
use crate::error::{BeanstalkcError, BeanstalkcResult};
use crate::job::Job;
use crate::request::Request;
use crate::response::Response;

/// `Beanstalkc` provides beanstalkd client operations.
#[derive(Debug)]
pub struct Beanstalkc {
    host: String,
    port: u16,
    connection_timeout: Option<Duration>,
    stream: Option<BufStream<TcpStream>>,
}

impl Beanstalkc {
    /// Create a new `Beanstalkc` instance with default configs.
    /// Default connection address is `localhost:11300`
    pub fn new() -> Beanstalkc {
        Beanstalkc {
            host: DEFAULT_HOST.to_string(),
            port: DEFAULT_PORT,
            connection_timeout: DEFAULT_CONNECTION_TIMEOUT,
            stream: None,
        }
    }

    /// Change host to beanstalkd server.
    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    /// Change port to beanstalkd server.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set timeout for TCP connection to beanstalkd server.
    /// Default connection timeout is `120s`.
    pub fn connection_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.connection_timeout = timeout;
        self
    }

    /// Connect to a running beanstalkd server.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```
    /// let conn = Beanstalkc::new().connect().unwrap();
    /// ```
    ///
    /// With custom configurations
    ///
    /// ```
    /// let mut conn = Beanstalkc::new()
    ///        .host("127.0.0.1")
    ///        .port(11300)
    ///        .connection_timeout(Some(time::Duration::from_secs(5)))
    ///        .connect()
    ///        .unwrap();
    /// ```
    pub fn connect(mut self) -> BeanstalkcResult<Self> {
        let addr = format!("{}:{}", self.host, self.port);
        let tcp_stream = match self.connection_timeout {
            Some(timeout) => {
                let addresses: Vec<_> = addr
                    .to_socket_addrs()
                    .unwrap_or_else(|_| panic!("failed to parse address: {}", addr))
                    .filter(|x| x.is_ipv4())
                    .collect();
                // FIXME: maybe we should try every possible addresses?
                TcpStream::connect_timeout(&addresses.first().unwrap(), timeout)?
            }
            None => TcpStream::connect(&addr)?,
        };
        self.stream = Some(BufStream::new(tcp_stream));
        Ok(self)
    }

    /// Close connection to remote server.
    fn close(&mut self) {}

    /// Re-connect to the beanstalkd server.
    pub fn reconnect(&mut self) {}

    /// Put a job into the current tube with default configs. Return job id.
    pub fn put_default(&mut self, body: &[u8]) -> BeanstalkcResult<u64> {
        self.put(
            body,
            DEFAULT_JOB_PRIORITY,
            DEFAULT_JOB_DELAY,
            DEFAULT_JOB_TTR,
        )
    }

    /// Put a job into the current tube and return the job id.
    pub fn put(
        &mut self,
        body: &[u8],
        priority: u32,
        delay: Duration,
        ttr: Duration,
    ) -> BeanstalkcResult<u64> {
        self.send(command::put(body, priority, delay, ttr))
            .map(|r| r.params[0])
    }

    /// Reserve a job from one of those watched tubes. Return a `Job` object if it succeeds.
    pub fn reserve(&mut self) -> BeanstalkcResult<Job> {
        let resp = self.send(command::reserve(None))?;
        Ok(Job::new(
            self,
            resp.params[0],
            resp.body.unwrap_or_default(),
            true,
        ))
    }

    /// Reserve a job with given timeout from one of those watched tubes.
    /// Return a `Job` object if it succeeds.
    pub fn reserve_with_timeout(&mut self, timeout: Duration) -> BeanstalkcResult<Job> {
        let resp = self.send(command::reserve(Some(timeout)))?;
        Ok(Job::new(
            self,
            resp.params[0],
            resp.body.unwrap_or_default(),
            true,
        ))
    }

    /// Kick at most `bound` jobs into the ready queue.
    pub fn kick(&mut self, bound: u32) -> BeanstalkcResult<u64> {
        self.send(command::kick(bound)).map(|r| r.params[0])
    }

    /// Kick a specific job into the ready queue.
    pub fn kick_job(&mut self, job_id: u64) -> BeanstalkcResult<()> {
        self.send(command::kick_job(job_id)).map(|_| ())
    }

    /// Return a specific job.
    pub fn peek(&mut self, job_id: u64) -> BeanstalkcResult<Job> {
        self.do_peek(command::peek_job(job_id))
    }

    /// Return the next ready job.
    pub fn peek_ready(&mut self) -> BeanstalkcResult<Job> {
        self.do_peek(command::peek_ready())
    }

    /// Return the delayed job with the shortest delay left.
    pub fn peek_delayed(&mut self) -> BeanstalkcResult<Job> {
        self.do_peek(command::peek_delayed())
    }

    /// Return the next job in the list of buried jobs.
    pub fn peek_buried(&mut self) -> BeanstalkcResult<Job> {
        self.do_peek(command::peek_buried())
    }

    pub fn do_peek(&mut self, cmd: command::Command) -> BeanstalkcResult<Job> {
        let resp = self.send(cmd)?;
        Ok(Job::new(
            self,
            resp.params[0],
            resp.body.unwrap_or_default(),
            false,
        ))
    }

    /// Return a list of all existing tubes.
    pub fn tubes(&mut self) -> BeanstalkcResult<Vec<String>> {
        self.send(command::tubes());
        Ok(vec![])
    }

    /// Return the tube currently being used.
    pub fn using(&mut self) -> BeanstalkcResult<String> {
        self.send(command::using());
        Ok("".to_string())
    }

    /// Use a given tube.
    pub fn use_tube(&mut self, name: &str) -> BeanstalkcResult<()> {
        self.send(command::use_tube(name));
        Ok(())
    }

    /// Return a list of tubes currently being watched.
    pub fn watching(&mut self) -> BeanstalkcResult<Vec<String>> {
        self.send(command::watching());
        Ok(vec![])
    }

    /// Watch a specific tube.
    pub fn watch(&mut self, name: &str) -> BeanstalkcResult<()> {
        self.send(command::watch(name));
        Ok(())
    }

    /// Stop watching a specific tube.
    pub fn ignore(&mut self, name: &str) -> BeanstalkcResult<()> {
        self.send(command::ignore(name));
        Ok(())
    }

    /// Return a dict of statistical information about the beanstalkd server.
    pub fn stats(&mut self) -> BeanstalkcResult<HashMap<String, String>> {
        self.send(command::stats()).map(|r|r.body_as_map())
    }

    /// Return a dict of statistical information about the specified tube.
    pub fn stats_tube(&mut self, name: &str) -> BeanstalkcResult<HashMap<String, String>> {
        self.send(command::stats_tube(name));
        Ok(HashMap::new())
    }

    /// Pause the specific tube for `delay` time.
    pub fn pause_tube(&mut self, name: &str, delay: Duration) -> BeanstalkcResult<()> {
        self.send(command::pause_tube(name, delay));
        Ok(())
    }

    /// Delete job by job id.
    pub fn delete(&mut self, job_id: u64) -> BeanstalkcResult<()> {
        self.send(command::delete(job_id));
        Ok(())
    }

    /// Release a reserved job back into the ready queue with default priority and delay.
    pub fn release_default(&mut self, job_id: u64) -> BeanstalkcResult<()> {
        self.release(job_id, DEFAULT_JOB_PRIORITY, DEFAULT_JOB_DELAY)
    }

    /// Release a reserved job back into the ready queue.
    pub fn release(&mut self, job_id: u64, priority: u32, delay: Duration) -> BeanstalkcResult<()> {
        self.send(command::release(job_id, priority, delay));
        Ok(())
    }

    /// Bury a specific job with default priority.
    pub fn bury_default(&mut self, job_id: u64) -> BeanstalkcResult<()> {
        self.bury(job_id, DEFAULT_JOB_PRIORITY)
    }

    /// Bury a specific job.
    pub fn bury(&mut self, job_id: u64, priority: u32) -> BeanstalkcResult<()> {
        self.send(command::bury(job_id, priority));
        Ok(())
    }

    /// Touch a job by `job_id`. Allowing the worker to request more time on a reserved
    /// job before it expires.
    pub fn touch(&mut self, job_id: u64) -> BeanstalkcResult<()> {
        self.send(command::touch(job_id));
        Ok(())
    }

    /// Return a dict of statistical information about a job.
    pub fn stats_job(&mut self, job_id: u64) -> BeanstalkcResult<HashMap<String, String>> {
        self.send(command::stats_job(job_id));
        Ok(HashMap::new())
    }

    fn send(&mut self, cmd: command::Command) -> BeanstalkcResult<Response> {
        let mut request = Request::new(self.stream.as_mut().unwrap());
        let resp = request.send(cmd.build().as_bytes())?;

        if cmd.expected_ok_status.contains(&resp.status) {
            Ok(resp)
        } else if cmd.expected_error_status.contains(&resp.status) {
            Err(BeanstalkcError::CommandFailed(format!("{:?}", resp.status)))
        } else {
            Err(BeanstalkcError::UnexpectedResponse(format!(
                "{:?}",
                resp.status
            )))
        }
    }
}

impl Drop for Beanstalkc {
    fn drop(&mut self) {
        self.close();
    }
}

impl Default for Beanstalkc {
    fn default() -> Self {
        Beanstalkc::new()
    }
}
