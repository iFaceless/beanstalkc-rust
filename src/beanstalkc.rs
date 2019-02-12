use std::collections::HashMap;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::time::Duration;

use bufstream::BufStream;

use crate::config::*;
use crate::errors::BeanstalkcResult;
use crate::job::Job;

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
    pub fn host(mut self, h: &str) -> Self {
        self.host = h.to_string();
        self
    }

    /// Change port to beanstalkd server.
    pub fn port(mut self, p: u16) -> Self {
        self.port = p;
        self
    }

    /// Set timeout for TCP connection to beanstalkd server.
    /// Default connection time is `120s`.
    pub fn connection_timeout(mut self, t: Option<Duration>) -> Self {
        self.connection_timeout = t;
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
        let addr: SocketAddr = format!("{}:{}", self.host, self.port).parse()?;
        let tcp_stream = match self.connection_timeout {
            Some(timeout) => TcpStream::connect_timeout(&addr, timeout)?,
            None => TcpStream::connect(&addr)?,
        };
        self.stream = Some(BufStream::new(tcp_stream));
        Ok(self)
    }

    /// Close connection to remote server.
    pub fn close(&mut self) {}

    /// Re-connect to the beanstalkd server.
    pub fn reconnect(&mut self) {}

    /// Put a job into the current tube with default configs. Return job id.
    pub fn put_default<T>(&self, body: T) -> BeanstalkcResult<u64> {
        self.put(
            body,
            DEFAULT_JOB_PRIORITY,
            DEFAULT_JOB_DELAY,
            DEFAULT_JOB_TTR,
        )
    }

    /// Put a job into the current tube and return the job id.
    pub fn put<T>(
        &self,
        body: T,
        priority: u32,
        delay: Duration,
        ttr: Duration,
    ) -> BeanstalkcResult<u64> {
        Ok(123)
    }

    /// Reserve a job from one of those watched tubes. Return a `Job` object if it succeeds.
    /// Timeout is optional with None.
    pub fn reserve(&self, timeout: Option<Duration>) -> BeanstalkcResult<Job> {
        Ok(Job::new(self, 0, String::new(), false))
    }

    /// Kick at most `bound` jobs into the ready queue.
    pub fn kick(&self, bound: u32) -> BeanstalkcResult<()> {
        Ok(())
    }

    /// Kick a specific job into the ready queue.
    pub fn kick_job(&self, job_id: u64) -> BeanstalkcResult<()> {
        Ok(())
    }

    /// Return a specific job.
    pub fn peek(&self, job_id: u64) -> BeanstalkcResult<Job> {
        Ok(Job::new(self, 0, String::new(), false))
    }

    /// Return the next ready job.
    pub fn peek_ready(&self) -> BeanstalkcResult<Job> {
        Ok(Job::new(self, 0, String::new(), false))
    }

    /// Return the delayed job with the shortest delay left.
    pub fn peek_delayed(&self) -> BeanstalkcResult<Job> {
        Ok(Job::new(self, 0, String::new(), false))
    }

    /// Return the next job in the list of buried jobs.
    pub fn peek_buried(&self) -> BeanstalkcResult<Job> {
        Ok(Job::new(self, 0, String::new(), false))
    }

    /// Return a list of all existing tubes.
    pub fn tubes(&self) -> BeanstalkcResult<Vec<String>> {
        Ok(vec![])
    }

    /// Return the tube currently being used.
    pub fn using(&self) -> BeanstalkcResult<String> {
        Ok("".to_string())
    }

    /// Use a given tube.
    pub fn use_tube(&self, name: &str) -> BeanstalkcResult<()> {
        Ok(())
    }

    /// Return a list of tubes currently being watched.
    pub fn watching(&self) -> BeanstalkcResult<Vec<String>> {
        Ok(vec![])
    }

    /// Watch a specific tube.
    pub fn watch(&self, name: &str) -> BeanstalkcResult<()> {
        Ok(())
    }

    /// Stop watching a specific tube.
    pub fn ignore(&self, name: &str) -> BeanstalkcResult<()> {
        Ok(())
    }

    /// Return a dict of statistical information about the beanstalkd server.
    pub fn stats(&self) -> BeanstalkcResult<HashMap<String, String>> {
        Ok(HashMap::new())
    }

    /// Return a dict of statistical information about the specified tube.
    pub fn stats_tube(&self, name: &str) -> BeanstalkcResult<HashMap<String, String>> {
        Ok(HashMap::new())
    }

    /// Pause the specific tube for `delay` time.
    pub fn pause_tube(&self, name: &str, delay: Duration) -> BeanstalkcResult<()> {
        Ok(())
    }

    /// Delete job by job id.
    pub fn delete(&self, job_id: u64) -> BeanstalkcResult<()> {
        Ok(())
    }

    /// Release a reserved job back into the ready queue with default priority and delay.
    pub fn release_default(&self, job_id: u64) -> BeanstalkcResult<()> {
        self.release(job_id, DEFAULT_JOB_PRIORITY, DEFAULT_JOB_DELAY)
    }

    /// Release a reserved job back into the ready queue.
    pub fn release(&self, job_id: u64, priority: u32, delay: Duration) -> BeanstalkcResult<()> {
        Ok(())
    }

    /// Bury a specific job with default priority.
    pub fn bury_default(&self, job_id: u64) -> BeanstalkcResult<()> {
        self.bury(job_id, DEFAULT_JOB_PRIORITY)
    }

    /// Bury a specific job.
    pub fn bury(&self, job_id: u64, priority: u32) -> BeanstalkcResult<()> {
        Ok(())
    }

    /// Touch a job by `job_id`. Allowing the worker to request more time on a reserved
    /// job before it expires.
    pub fn touch(&self, job_id: u64) -> BeanstalkcResult<()> {
        Ok(())
    }

    /// Return a dict of statistical information about a job.
    pub fn stats_job(&self, job_id: u64) -> BeanstalkcResult<HashMap<String, String>> {
        Ok(HashMap::new())
    }
}

impl Drop for Beanstalkc {
    fn drop(&mut self) {
        self.close();
    }
}
