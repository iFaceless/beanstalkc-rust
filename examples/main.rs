use beanstalkc::Beanstalkc;
use std::time;

fn main() {
    let mut conn = Beanstalkc::new()
        .host("127.0.0.1")
        .port(11300)
        .connection_timeout(Some(time::Duration::from_secs(10)))
        .connect()
        .expect("connection failed");

    conn.use_tube("urls").unwrap();
    conn.put_default("hello").unwrap();

    conn.watch("urls").unwrap();
    conn.reserve().unwrap();
}
