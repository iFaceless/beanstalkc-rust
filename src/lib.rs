//! A simple Beanstalkd client.
//!
//! This crate provides a simple and easy-to-use beanstalkd client, which is inspired
//! by [beanstalkc](https://github.com/earl/beanstalkc/) and [rust-beanstalkd](https://github.com/schickling/rust-beanstalkd).
//!
//! # Usage
//!
//! ```toml
//! [dependencies]
//! beanstalkc-rust = "0.1.0"
//! ```
//!
//! Producer
//!
//! ```no_run
//! use time::Duration;
//! use beanstalkc::Beanstalkc;
//!
//! let mut conn = Beanstalkc::new()
//!      .connect()
//!      .expect("connect to beanstalkd server failed");
//!
//! conn.use_tube("jobs").unwrap();
//! conn.put_default("hello, world").unwrap();
//! conn.put("hello, rust", 1, Duration::from_secs(10), Duration::from_secs(1800)).unwrap();
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

mod beanstalkc;
mod command;
mod config;
mod error;
mod job;
mod request;
mod response;
