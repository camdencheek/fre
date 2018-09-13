use rand::prelude::*;
use std::io::Write;
use tempfile;
use topd::{store,stats};
use rayon::prelude::*;
use std::time::SystemTime;

pub fn get_tempfile_path() -> tempfile::TempPath {

    let mut file = tempfile::NamedTempFile::new().unwrap();


    file.write(r#"{
      "reference_time": {
        "secs_since_epoch": 1536521851,
        "nanos_since_epoch":885366138},
      "half_life_secs":1209600,
      "directories": [
        {
          "directory": "/home",
          "frecency": 3.0,
          "last_accessed":10,
          "num_accesses": 2
        },
        {
          "directory": "/home/nonexistant_dir",
          "frecency": 2.0,
          "last_accessed":30,
          "num_accesses": 1
        },
        {
          "directory": "/",
          "frecency": 1.0,
          "last_accessed":20,
          "num_accesses":3
        }
      ]
    }"#
        .as_bytes()).unwrap();

    return file.into_temp_path()
}

fn random_usage(n: u64) -> topd::store::Store {
    let mut usage = store::Store::default();
    let ref_time = SystemTime::now();
    for i in 0..n {
      let dir = format!("/home/ccheek/test/test{}", i).to_string();
      usage.add(dir.clone());
      let new_dir = usage.find_mut(&dir).unwrap();

      new_dir.last_accessed = random();
      new_dir.frecency = random();
      new_dir.num_accesses = random();
    }

    return usage
}
