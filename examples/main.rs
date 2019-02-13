use beanstalkc::Beanstalkc;
use std::time;

fn main() {
    let mut conn = Beanstalkc::new()
        .host("localhost")
        .port(11301)
        .connection_timeout(Some(time::Duration::from_secs(1)))
        .connect()
        .expect("connection failed");

    conn.put_default(b"hello");
    conn.put_default("world".as_bytes());
    conn.put_default(String::from("rust beanstalkd").as_bytes());

    conn.reserve();
    conn.reserve_with_timeout(time::Duration::from_secs(10));
}
