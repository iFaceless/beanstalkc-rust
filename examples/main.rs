extern crate flate2;

use std::error::Error;
use std::time;
use std::io::prelude::*;

use beanstalkc::Beanstalkc;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;

fn main() -> Result<(), Box<dyn Error>> {
    let mut conn = Beanstalkc::new()
        .host("localhost")
        .port(11300)
        .connection_timeout(Some(time::Duration::from_secs(1)))
        .connect()
        .expect("connection failed");

    dbg!(conn.put_default(b"hello"))?;
    dbg!(conn.put(
        b"Hello, rust world.",
        0,
        time::Duration::from_secs(100),
        time::Duration::from_secs(1800)
    ))?;
    dbg!(conn.reserve())?;
    dbg!(conn.kick(100))?;
    dbg!(conn.kick_job(10))?;
    dbg!(conn.peek(10))?;
    dbg!(conn.peek_ready())?;
    dbg!(conn.peek_buried())?;
    dbg!(conn.peek_delayed())?;
    dbg!(conn.tubes())?;
    dbg!(conn.using())?;
    dbg!(conn.use_tube("jobs"))?;
    dbg!(conn.watch("jobs"))?;
    dbg!(conn.watching())?;
    dbg!(conn.ignore("jobs"))?;
    dbg!(conn.ignore("default"))?;
    dbg!(conn.stats_tube("default"))?;
    dbg!(conn.pause_tube("jobs", time::Duration::from_secs(10)))?;
    dbg!(conn.pause_tube("not-found", time::Duration::from_secs(10)))?;

    let mut job = conn.reserve()?;
    dbg!(job.id());
    dbg!(std::str::from_utf8(job.body()))?;
    dbg!(job.reserved());
    dbg!(job.bury_default())?;
    dbg!(job.kick())?;
    dbg!(job.touch())?;
    dbg!(job.stats())?;
    dbg!(job.touch())?;
    dbg!(job.release_default())?;
    dbg!(job.delete())?;

    let mut job = conn.reserve()?;
    dbg!(job.delete())?;

    // should also work with potentially non-UTF-8 payloads
    // puts a gzip encoded message
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(b"Hello beanstalkc compressed")?;
    let buf = e.finish()?;
    dbg!(conn.put_default(&buf))?;

    // tries to read the gzipped encoded message back to a string
    let mut job = conn.reserve()?;
    let mut buf = &job.body().to_owned()[..];
    let mut gz = GzDecoder::new(&mut buf);
    let mut s = String::new();
    gz.read_to_string(&mut s)?;
    dbg!(s);
    job.delete()?;

    let mut conn = conn.reconnect()?;
    dbg!(conn.stats())?;

    let stats = conn.stats()?;
    dbg!(stats);

    Ok(())
}
