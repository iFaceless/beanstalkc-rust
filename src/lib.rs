mod beanstalkc;
mod command;
mod errors;
mod request;
mod response;

pub use crate::beanstalkc::Beanstalkc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
