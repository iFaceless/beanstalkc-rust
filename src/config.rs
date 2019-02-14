use std::time::Duration;

/// Default configurations for Beanstalkd client.
pub const DEFAULT_HOST: &str = "localhost";
pub const DEFAULT_PORT: u16 = 11300;
pub const DEFAULT_CONNECTION_TIMEOUT: Option<Duration> = Some(Duration::from_secs(120));
pub const DEFAULT_JOB_PRIORITY: u32 = 1 << 31;
pub const DEFAULT_JOB_TTR: Duration = Duration::from_secs(120);
pub const DEFAULT_JOB_DELAY: Duration = Duration::from_secs(0);
