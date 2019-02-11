use beanstalkc::Beanstalkc;
use std::time;

fn main() {
    let mut client = Beanstalkc::new()
        .host("127.0.0.1")
        .port(11300)
        .timeout(time::Duration::from_secs(10))
        .connect()
        .unwrap();

    client.use_("urls").unwrap();
}
