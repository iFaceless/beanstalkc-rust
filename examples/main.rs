use beanstalkc::Beanstalkc;
use std::time;

fn main() {
    let mut client = Beanstalkc::new()
        .host("127.0.0.1")
        .port(11300)
        .connection_timeout(Some(time::Duration::from_secs(10)))
        .connect()
        .unwrap();

    client.use_tube("urls").unwrap();
}
