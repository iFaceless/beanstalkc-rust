pub use crate::beanstalkc::Beanstalkc;

mod beanstalkc;
mod config;
mod errors;
mod job;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
