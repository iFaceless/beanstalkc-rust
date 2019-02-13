use beanstalkc::Beanstalkc;
use std::time;

fn main() {
    let mut conn = Beanstalkc::new()
        .host("localhost")
        .port(11301)
        .connection_timeout(Some(time::Duration::from_secs(1)))
        .connect()
        .expect("connection failed");

    println!("{:?}", conn.put_default(b"hello"));
    println!("{}", conn.reserve().unwrap());
    println!("{:?}", conn.peek_buried());
    print!("{:#?}", conn.stats());
}
