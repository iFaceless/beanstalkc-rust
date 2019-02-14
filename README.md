Beanstalkd Client for Rust
=========================

[Beanstalkd](https://github.com/beanstalkd/beanstalkd) is a fast, general-purpose work queue. [beanstalkc-rust](https://github.com/iFaceless/beanstalkc-rust) is a Beanstalkd Client to communicate with Beanstalkd Server based on the protocol defined [here](./protocol.md).

*Inspired by [rust-beanstalkd](https://github.com/schickling/rust-beanstalkd) and [beanstalkc](https://github.com/earl/beanstalkc/).*

# Why

Several repositories can be found from [here](https://github.com/search?q=beanstalkd+rust), why not just using one of those directly? The reasons are as follows:
1. Some of them were poorly documented;
1. Some of them were not actively developed or maintained;
1. This [rust-beanstalkd](https://github.com/schickling/rust-beanstalkd) repo with the most stars was already out-dated, since not all the beanstalkd commands were supported.

# Features

1. Easy to use;
1. Support custom connection timeout;
1. Support all the commands defined in the [protocol.txt](https://github.com/beanstalkd/beanstalkd/blob/master/doc/protocol.txt);
1. Well documented.

# Install

Add this dependency to your `Cargo.toml`~

```
beanstalkc = "^0.2.0"
```

# Documentation

Full documentation can be found [here](https://docs.rs/beanstalkc/0.2.0/beanstalkc).

# Usage

More examples can be found [here](./examples/main.rs).

## Producer
```rust
use beanstalkc::Beanstalkc;
use std::time;

fn main() {
    let mut conn = Beanstalkc::new()
        .host("127.0.0.1")
        .port(11300)
        .connection_timeout(Some(time::Duration::from_secs(10)))
        .connect()
        .expect("connection failed");

    conn.use_tube("jobs").unwrap();

    conn.put_default(b"hello, world").unwrap();
    conn.put(
            b"Hello, rust world.",
            0,
            time::Duration::from_secs(100),
            time::Duration::from_secs(1800)
        )
}
```

## Consumer

```rust
use beanstalkc::Beanstalkc;
use std::time;

fn main() {
    let mut conn = Beanstalkc::new()
        .host("127.0.0.1")
        .port(11300)
        .connection_timeout(Some(time::Duration::from_secs(10)))
        .connect()
        .expect("connection failed");

    conn.watch("jobs").unwrap();

    let mut job = conn.reserve().unwrap();
    println!("{:#?}", job.stats());

    job.delete().unwrap();
}
```

# License

Licensed under the [MIT license](./LICENSE)

# Contribution

Please feel free to report any issues~