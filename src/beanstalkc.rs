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
    ///
    /// # Example:
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().host("localhost").connect().unwrap();
    /// ```
    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    /// Change port to beanstalkd server.
    ///
    /// # Example:
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().port(12345).connect().unwrap();
    /// ```
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set timeout for TCP connection to beanstalkd server.
    /// Default connection timeout is `120s`.
    ///
    /// # Example:
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new()
    ///        .connection_timeout(Some(Duration::from_secs(10)))
    ///        .connect()
    ///        .unwrap();
    /// ```
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
    /// ```no_run
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    /// ```
    ///
    /// With custom configurations
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new()
    ///        .host("127.0.0.1")
    ///        .port(11300)
    ///        .connection_timeout(Some(Duration::from_secs(5)))
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
    #[allow(unused_must_use)]
    fn close(&mut self) {
        self.send(command::quit());
    }

    /// Re-connect to the beanstalkd server.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    /// let mut conn = conn.reconnect().unwrap();
    /// ```
    pub fn reconnect(mut self) -> BeanstalkcResult<Self> {
        self.close();
        self.connect()
    }

    /// Put a job into the current tube with default configs. Return job id.
    ///
    /// # Example:
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// let job_id = conn.put_default(b"Rust").unwrap();
    /// ```
    pub fn put_default(&mut self, body: &[u8]) -> BeanstalkcResult<u64> {
        self.put(
            body,
            DEFAULT_JOB_PRIORITY,
            DEFAULT_JOB_DELAY,
            DEFAULT_JOB_TTR,
        )
    }

    /// Put a job into the current tube and return the job id.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// let job_id = conn.put(
    ///        b"Rust",
    ///        0,
    ///        Duration::from_secs(1),
    ///        Duration::from_secs(10),
    ///    );
    /// ```
    pub fn put(
        &mut self,
        body: &[u8],
        priority: u32,
        delay: Duration,
        ttr: Duration,
    ) -> BeanstalkcResult<u64> {
        self.send(command::put(body, priority, delay, ttr))
            .and_then(|r| r.job_id())
    }

    /// Reserve a job from one of those watched tubes. Return a `Job` object if it succeeds.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// let mut job = conn.reserve().unwrap();
    /// // Execute job...
    /// dbg!(job.id());
    /// dbg!(job.body());
    ///
    /// job.delete().unwrap();
    /// ```
    pub fn reserve(&mut self) -> BeanstalkcResult<Job> {
        let resp = self.send(command::reserve(None))?;
        Ok(Job::new(
            self,
            resp.job_id()?,
            resp.body.unwrap_or_default(),
            true,
        ))
    }

    /// Reserve a job with given timeout from one of those watched tubes.
    /// Return a `Job` object if it succeeds.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    /// use std::time::Duration;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// let mut job = conn.reserve_with_timeout(Duration::from_secs(10)).unwrap();
    /// // Execute job...
    /// dbg!(job.id());
    /// dbg!(job.body());
    ///
    /// job.delete().unwrap();
    /// ```
    pub fn reserve_with_timeout(&mut self, timeout: Duration) -> BeanstalkcResult<Job> {
        let resp = self.send(command::reserve(Some(timeout)))?;
        Ok(Job::new(
            self,
            resp.job_id()?,
            resp.body.unwrap_or_default(),
            true,
        ))
    }

    /// Kick at most `bound` jobs into the ready queue.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// assert_eq!(10, conn.kick(10).unwrap());
    /// ```
    pub fn kick(&mut self, bound: u32) -> BeanstalkcResult<u64> {
        self.send(command::kick(bound))
            .and_then(|r| r.get_int_param(0))
    }

    /// Kick a specific job into the ready queue.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// conn.kick(123).unwrap();
    /// ```
    pub fn kick_job(&mut self, job_id: u64) -> BeanstalkcResult<()> {
        self.send(command::kick_job(job_id)).map(|_| ())
    }

    /// Return a specific job.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// let mut job = conn.peek(1).unwrap();
    /// assert_eq!(1, job.id());
    /// ```
    pub fn peek(&mut self, job_id: u64) -> BeanstalkcResult<Job> {
        self.do_peek(command::peek_job(job_id))
    }

    /// Return the next ready job.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// let mut job = conn.peek_ready().unwrap();
    /// dbg!(job.id());
    /// dbg!(job.body());
    /// ```
    pub fn peek_ready(&mut self) -> BeanstalkcResult<Job> {
        self.do_peek(command::peek_ready())
    }

    /// Return the delayed job with the shortest delay left.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// let mut job = conn.peek_delayed().unwrap();
    /// dbg!(job.id());
    /// dbg!(job.body());
    /// ```
    pub fn peek_delayed(&mut self) -> BeanstalkcResult<Job> {
        self.do_peek(command::peek_delayed())
    }

    /// Return the next job in the list of buried jobs.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// let mut job = conn.peek_buried().unwrap();
    /// dbg!(job.id());
    /// dbg!(job.body());
    /// ```
    pub fn peek_buried(&mut self) -> BeanstalkcResult<Job> {
        self.do_peek(command::peek_buried())
    }

    pub fn do_peek(&mut self, cmd: command::Command) -> BeanstalkcResult<Job> {
        let resp = self.send(cmd)?;
        Ok(Job::new(
            self,
            resp.job_id()?,
            resp.body.unwrap_or_default(),
            false,
        ))
    }

    /// Return a list of all existing tubes.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// let tubes = conn.tubes().unwrap();
    /// assert!(tubes.contains(&String::from("default")));
    /// ```
    pub fn tubes(&mut self) -> BeanstalkcResult<Vec<String>> {
        self.send(command::tubes()).map(|r| r.body_as_vec())
    }

    /// Return the tube currently being used.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// let tube = conn.using().unwrap();
    /// assert_eq!("default".to_string(), tube);
    /// ```
    pub fn using(&mut self) -> BeanstalkcResult<String> {
        self.send(command::using()).and_then(|r| r.get_param(0))
    }

    /// Use a given tube.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// let tube = conn.use_tube("jobs").unwrap();
    /// assert_eq!("jobs".to_string(), tube);
    /// ```
    pub fn use_tube(&mut self, name: &str) -> BeanstalkcResult<String> {
        self.send(command::use_tube(name))
            .and_then(|r| r.get_param(0))
    }

    /// Return a list of tubes currently being watched.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// let tubes = conn.watching().unwrap();
    /// assert_eq!(vec!["default".to_string()], tubes);
    /// ```
    pub fn watching(&mut self) -> BeanstalkcResult<Vec<String>> {
        self.send(command::watching()).map(|r| r.body_as_vec())
    }

    /// Watch a specific tube.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// let watched_count = conn.watch("foo").unwrap();
    /// assert_eq!(2, watched_count);
    /// ```
    pub fn watch(&mut self, name: &str) -> BeanstalkcResult<u64> {
        self.send(command::watch(name))
            .and_then(|r| r.get_int_param(0))
    }

    /// Stop watching a specific tube.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    /// conn.ignore("foo").unwrap();
    /// ```
    pub fn ignore(&mut self, name: &str) -> BeanstalkcResult<u64> {
        self.send(command::ignore(name))
            .and_then(|r| r.get_int_param(0))
    }

    /// Return a dict of statistical information about the beanstalkd server.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// dbg!(conn.stats().unwrap());
    /// ```
    pub fn stats(&mut self) -> BeanstalkcResult<HashMap<String, String>> {
        self.send(command::stats()).map(|r| r.body_as_map())
    }

    /// Return a dict of statistical information about the specified tube.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// dbg!(conn.stats_tube("default").unwrap());
    /// ```
    pub fn stats_tube(&mut self, name: &str) -> BeanstalkcResult<HashMap<String, String>> {
        self.send(command::stats_tube(name))
            .map(|r| r.body_as_map())
    }

    /// Pause the specific tube for `delay` time.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    /// conn.pause_tube("default", Duration::from_secs(100));
    /// ```
    pub fn pause_tube(&mut self, name: &str, delay: Duration) -> BeanstalkcResult<()> {
        self.send(command::pause_tube(name, delay)).map(|_| ())
    }

    /// Delete job by job id.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// let job = conn.reserve().unwrap();
    /// conn.delete(job.id).unwrap();
    ///
    /// let mut job = conn.reserve().unwrap();
    /// // Recommended way to delete a job
    /// job.delete().unwrap();
    /// ```
    pub fn delete(&mut self, job_id: u64) -> BeanstalkcResult<()> {
        self.send(command::delete(job_id)).map(|_| ())
    }

    /// Release a reserved job back into the ready queue with default priority and delay.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// conn.release_default(1).unwrap();
    /// ```
    pub fn release_default(&mut self, job_id: u64) -> BeanstalkcResult<()> {
        self.release(job_id, DEFAULT_JOB_PRIORITY, DEFAULT_JOB_DELAY)
    }

    /// Release a reserved job back into the ready queue.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// conn.release(1, 0, Duration::from_secs(10)).unwrap();
    /// ```
    pub fn release(&mut self, job_id: u64, priority: u32, delay: Duration) -> BeanstalkcResult<()> {
        self.send(command::release(job_id, priority, delay))
            .map(|_| ())
    }

    /// Bury a specific job with default priority.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// conn.bury_default(1).unwrap();
    /// ```
    pub fn bury_default(&mut self, job_id: u64) -> BeanstalkcResult<()> {
        self.bury(job_id, DEFAULT_JOB_PRIORITY)
    }

    /// Bury a specific job.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// conn.bury(1, 0).unwrap();
    /// ```
    pub fn bury(&mut self, job_id: u64, priority: u32) -> BeanstalkcResult<()> {
        self.send(command::bury(job_id, priority)).map(|_| ())
    }

    /// Touch a job by `job_id`. Allowing the worker to request more time on a reserved
    /// job before it expires.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// conn.touch(1).unwrap();
    /// ```
    pub fn touch(&mut self, job_id: u64) -> BeanstalkcResult<()> {
        self.send(command::touch(job_id)).map(|_| ())
    }

    /// Return a dict of statistical information about a job.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beanstalkc::Beanstalkc;
    ///
    /// let mut conn = Beanstalkc::new().connect().unwrap();
    ///
    /// let stats = conn.stats_job(1).unwrap();
    /// dbg!(stats);
    /// ```
    pub fn stats_job(&mut self, job_id: u64) -> BeanstalkcResult<HashMap<String, String>> {
        self.send(command::stats_job(job_id))
            .map(|r| r.body_as_map())
    }

    fn send(&mut self, cmd: command::Command) -> BeanstalkcResult<Response> {
        if self.stream.is_none() {
            return Err(BeanstalkcError::ConnectionError(
                "invalid connection".to_string(),
            ));
        }

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
