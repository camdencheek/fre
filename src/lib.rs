#[macro_use]
extern crate serde_derive;

use std::time::SystemTime;

pub mod args;
pub mod stats;
pub mod store;

/// Return the current time in seconds as a float
pub fn current_time_secs() -> f64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("failed to get system time")
        .as_secs_f64()
}

#[macro_export]
macro_rules! error_and_exit {
    ($($arg:tt)*) => {
        {
            error!($($arg)*);
            ::std::process::exit(1);
        }
    };
}
