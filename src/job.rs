use std::collections::HashMap;
use std::time::Duration;

use crate::Beanstalkc;
use crate::config::DEFAULT_JOB_DELAY;
use crate::config::DEFAULT_JOB_PRIORITY;
use crate::errors::BeanstalkcResult;

/// Job is a simple abstraction about beanstalkd job.
#[derive(Debug)]
pub struct Job<'a> {
    conn: &'a Beanstalkc,
    job_id: u64,
    body: String,
    reserved: bool,
}

impl<'a> Job<'a> {
    /// Initialize and return the `Job` object.
    pub fn new(conn: &'a Beanstalkc, job_id: u64, body: String, reserved: bool) -> Job {
        Job {
            conn,
            job_id,
            body,
            reserved,
        }
    }

    /// Delete this job.
    pub fn delete(&mut self) -> BeanstalkcResult<()> {
        self.conn.delete(self.job_id)?;
        self.reserved = false;
        Ok(())
    }

    /// Release this job back to the ready queue with default priority and delay.
    pub fn release_default(&mut self) -> BeanstalkcResult<()> {
        self.release(self.priority(), DEFAULT_JOB_DELAY)
    }

    /// Release this job back to the ready queue with custom priority and delay.
    pub fn release(&mut self, priority: u32, delay: Duration) -> BeanstalkcResult<()> {
        if !self.reserved {
            return Ok(());
        }

        self.conn.release(self.job_id, priority, delay)?;
        self.reserved = false;
        Ok(())
    }

    /// Bury this job with default priority.
    pub fn bury_default(&mut self) -> BeanstalkcResult<()> {
        self.bury(self.priority())
    }

    /// Bury this job with custom priority.
    pub fn bury(&mut self, priority: u32) -> BeanstalkcResult<()> {
        if !self.reserved {
            return Ok(());
        }

        self.conn.bury(self.job_id, priority)?;
        self.reserved = false;
        Ok(())
    }

    /// Kick this job to ready queue.
    pub fn kick(&self) -> BeanstalkcResult<()> {
        self.conn.kick_job(self.job_id)
    }

    /// Touch this reserved job, requesting more time to work on it.
    pub fn touch(&self) -> BeanstalkcResult<()> {
        if !self.reserved {
            return Ok(());
        }

        self.conn.touch(self.job_id)
    }

    /// Return a dict of statistical information about this job.
    pub fn stats(&self) -> BeanstalkcResult<HashMap<String, String>> {
        return self.conn.stats_job(self.job_id);
    }

    /// Return the job priority from this job stats. If not found, return the `DEFAULT_JOB_PRIORITY`.
    fn priority(&self) -> u32 {
        let stats = self.stats()
            .unwrap_or_else(|_| HashMap::new());
        match stats.get("pri") {
            None => DEFAULT_JOB_PRIORITY,
            Some(pri) => pri.parse().unwrap_or(DEFAULT_JOB_PRIORITY),
        }
    }
}
