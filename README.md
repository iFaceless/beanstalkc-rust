Beanstalkd Client for Rust
=========================

[Beanstalkd](https://github.com/beanstalkd/beanstalkd) is a fast, general-purpose work queue. [beanstalkc-rust](https://github.com/iFaceless/beanstalkc-rust) is a Beanstalkd Client to communicate with Beanstalkd Server based on the protocol defined [here](./protocol.md).

*Inspired by project [rust-beanstalkd](https://github.com/schickling/rust-beanstalkd)*

# Why another ONE

Several repos can be found from [here](https://github.com/search?q=beanstalkd+rust), why not just using one of those directly? The reasons are as follows:
1. Some of them were poorly documented.
1. Some of them were not actively developed or maintained.
1. This [rust-beanstalkd](https://github.com/schickling/rust-beanstalkd) repo with the most stars was already out-dated, since not all the beanstalkd commands were supported.

# Features

1. Easy to use
1. Support custom connection timeout
1. Support all the commands defined in the [protocol.txt](https://github.com/beanstalkd/beanstalkd/blob/master/doc/protocol.txt)
1. Well documented

# Documentation

Full documentation can be found [here]().

# Usage
## Producer
```rust
use beanstalkc;

fn main() {
    let client = beanstalkc::connect("localhost", 11300).unwrap();
    client.use("jobs")
}
```

## Consumer

```rust
use beanstalkc;

fn main() {
    let client = beanstalkc::connect("localhost", 11300).unwrap();
    client.use("jobs")
}
```

# License

Licensed under the [MIT license](./LICENSE)

# Contribution

Please feel free to report any issues~