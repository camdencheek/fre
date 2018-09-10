use rand::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile;

pub fn get_tempfile_path() -> tempfile::TempPath {

    let mut file = tempfile::NamedTempFile::new().unwrap();


    file.write(r#"{
      "time_created": {
        "secs_since_epoch": 1536521851,
        "nanos_since_epoch":885366138},
      "half_life_secs":1209600,
      "directories": {
        "/home": {
          "score": 3.0,
          "last_accessed":10,
          "num_accesses": 2
        },
        "/home/nonexistant_dir": {
          "score": 2.0,
          "last_accessed":30,
          "num_accesses": 1
        },
        "/": {
          "score": 1.0,
          "last_accessed":20,
          "num_accesses":3
        }
      }
    }"#
        .as_bytes()).unwrap();

    return file.into_temp_path()
}
