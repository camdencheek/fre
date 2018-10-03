#[macro_use]
extern crate serde_derive;

use directories::ProjectDirs;

use std::path::PathBuf;

pub mod args;
pub mod stats;
pub mod store;

#[derive(Debug)]
pub enum SortMethod {
    Recent,
    Frequent,
    Frecent,
}

pub fn default_store_path() -> PathBuf {
    let store_dir = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
        .expect("Cannot get default project directory")
        .data_dir()
        .to_path_buf();

    let mut store_file = store_dir.clone();
    store_file.push(format!("{}.json", env!("CARGO_PKG_NAME")));

    return store_file.to_path_buf();
}
