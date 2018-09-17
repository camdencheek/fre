mod common;

mod weight {
  use super::common;
  use assert_cmd::prelude::*;
  use std::process::Command;
  use predicates::prelude::*;
  use topd::store;
  use std::path::PathBuf;

  #[test]
  fn add_existing() {
    let store_file = common::get_tempfile_path();
    let dir = "/home".to_string();

    Command::main_binary()
      .unwrap()
      .arg("--store")
      .arg(&store_file.as_os_str())
      .arg("--add")
      .arg(&dir)
      .assert();

    let mut usage = store::read_store(&store_file.to_path_buf()).unwrap();

    assert!(usage.find(&dir).is_some());
    assert_eq!(3, usage.find(&dir).unwrap().num_accesses, "Number of accesses did not increment");
    assert!(usage.find(&dir).unwrap().frecency > 3.1, "Frecency did not increase");
    assert!(usage.find(&dir).unwrap().last_accessed > 11, "Last accessed time did not increase");
  }

  #[test]
  fn add_create() {
    let store_file = common::get_tempfile_path();
    let new_dir = "/home/super_new_dir".to_string();

    Command::main_binary()
      .unwrap()
      .arg("--store")
      .arg(&store_file.as_os_str())
      .arg("--add")
      .arg(&new_dir)
      .assert();

    let usage = store::read_store(&store_file.to_path_buf()).unwrap();

    assert!(usage.find(&new_dir).is_some());
    assert_eq!(usage.find(&new_dir).unwrap().num_accesses, 1, "Incorrect number of accesses");
    assert!(usage.find(&new_dir).unwrap().frecency > 0.1, "Frecency not greater than zero");
    assert!(usage.find(&new_dir).unwrap().last_accessed > 1, "Last accessed time not greater than zero");
  }

  #[test]
  fn add_relative() {
    let store_file = common::get_tempfile_path();
    let relative_dir = "./random_relative_dir".to_string();
    let mut absolute_dir = std::env::temp_dir();
    absolute_dir.push("random_relative_dir");
    let absolute_dir = absolute_dir
      .into_os_string()
      .into_string()
      .unwrap();


    Command::main_binary()
      .unwrap()
      .current_dir(std::env::temp_dir().as_os_str())
      .arg("--store")
      .arg(&store_file.as_os_str())
      .arg("--add")
      .arg(&relative_dir)
      .assert();

    let usage = store::read_store(&store_file.to_path_buf()).unwrap();

    assert!(usage.find(&absolute_dir).is_some());
  }
}
