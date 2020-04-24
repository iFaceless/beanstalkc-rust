//! A simple Beanstalkd client.
//!
//! This crate provides a simple and easy-to-use beanstalkd client, which is inspired
//! by [beanstalkc](https://github.com/earl/beanstalkc/) and [rust-beanstalkd](https://github.com/schickling/rust-beanstalkd).
//!
//! # Usage
//!
//! ```toml
//! [dependencies]
//! beanstalkc = "^0.2.0"
//! ```
//!
//! Producer
//!
//! ```no_run
//! use std::time::Duration;
//! use beanstalkc::Beanstalkc;
//!
//! let mut conn = Beanstalkc::new()
//!      .connect()
//!      .expect("connect to beanstalkd server failed");
//!
//! conn.use_tube("jobs").unwrap();
//! conn.put_default(b"hello, world").unwrap();
//! conn.put(b"hello, rust", 1, Duration::from_secs(10), Duration::from_secs(1800)).unwrap();
//! ```
//!
//! Worker
//!
//! ```no_run
//! use beanstalkc::Beanstalkc;
//!
//! let mut conn = Beanstalkc::new()
//!      .connect()
//!      .expect("connect to beanstalkd server failed");
//!
//! conn.watch("jobs").unwrap();
//!
//! let mut job = conn.reserve().unwrap();
//! // execute job here...
//! job.delete().unwrap();
//! ```
pub use crate::beanstalkc::Beanstalkc;
pub use crate::error::{BeanstalkcError, BeanstalkcResult};
pub use crate::job::Job;

mod beanstalkc;
mod command;
mod config;
mod error;
mod job;
mod request;
mod response;
