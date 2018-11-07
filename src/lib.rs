#![feature(duration_as_u128)]
#![feature(test)]

extern crate test;

#[macro_use]
extern crate serde_derive;

use std::time::SystemTime;
use log::error;
use std::process;

pub mod args;
pub mod stats;
pub mod store;

#[derive(Debug)]
pub enum SortMethod {
    Recent,
    Frequent,
    Frecent,
}

/// Return the current time in seconds as a float
pub fn current_time_secs() -> f64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_millis() as f64 / 1000.0,
        Err(e) => {
            error!("invalid system time: {}", e);
            process::exit(1);
        }
    }
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
