use beanstalkc::Beanstalkc;
use std::time;

fn main() {
    let mut conn = Beanstalkc::new()
        .host("localhost")
        .port(11300)
        .connection_timeout(Some(time::Duration::from_secs(1)))
        .connect()
        .expect("connection failed");

    dbg!(conn.put_default(b"hello"));
    dbg!(conn.put(
        b"Hello, rust world.",
        0,
        time::Duration::from_secs(100),
        time::Duration::from_secs(1800)
    ));
    dbg!(conn.reserve());
    dbg!(conn.kick(100));
    dbg!(conn.kick_job(10));
    dbg!(conn.peek(10));
    dbg!(conn.peek_ready());
    dbg!(conn.peek_buried());
    dbg!(conn.peek_delayed());
    dbg!(conn.tubes());
    dbg!(conn.using());
    dbg!(conn.use_tube("jobs"));
    dbg!(conn.watch("jobs"));
    dbg!(conn.watching());
    dbg!(conn.ignore("jobs"));
    dbg!(conn.ignore("default"));
    dbg!(conn.stats_tube("default"));
    dbg!(conn.pause_tube("jobs", time::Duration::from_secs(10)));
    dbg!(conn.pause_tube("not-found", time::Duration::from_secs(10)));

    let mut job = conn.reserve().unwrap();
    dbg!(job.id());
    dbg!(job.body());
    dbg!(job.reserved());
    dbg!(job.bury_default());
    dbg!(job.kick());
    dbg!(job.touch());
    dbg!(job.stats());
    dbg!(job.touch());
    dbg!(job.release_default());
    dbg!(job.delete());

    let mut conn = conn.reconnect().unwrap();
    dbg!(conn.stats());

    let stats = conn.stats().unwrap();
    dbg!(stats);
}
