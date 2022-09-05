use std::process::abort;

use log::error;

pub trait LogExpect<T> {
    fn log_expect(self, msg: &str) -> T;
}

impl<T, E> LogExpect<T> for Result<T, E> {
    fn log_expect(self, message: &str) -> T {
        match self {
            Ok(inner) => inner,
            Err(_error) => {
                error!("{}", message);
                abort();
            }
        }
    }
}

impl<T> LogExpect<T> for Option<T> {
    fn log_expect(self, msg: &str) -> T {
        match self {
            Some(inner) => inner,
            None => {
                error!("{}", msg);
                abort();
            }
        }
    }
}
