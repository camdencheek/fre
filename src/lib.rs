#[macro_use]
extern crate serde_derive;

pub mod args;
pub mod stats;
pub mod store;

#[derive(Debug)]
pub enum SortMethod {
    Recent,
    Frequent,
    Frecent,
}

