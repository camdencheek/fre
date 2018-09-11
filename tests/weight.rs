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

    Command::main_binary()
      .unwrap()
      .arg("--store")
      .arg(&store_file.as_os_str())
      .arg("--add")
      .arg("/home")
      .assert();

    let usage = store::read_store(&store_file.to_path_buf());

    assert!(usage.directories.contains_key("/home"));
    assert_eq!(3, usage.directories["/home"].num_accesses, "Number of accesses did not increment");
    assert!(usage.directories["/home"].score > 3.1, "Score did not increase");
    assert!(usage.directories["/home"].last_accessed > 11, "Last accessed time did not increase");
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

    let usage = store::read_store(&store_file.to_path_buf());

    assert!(usage.directories.contains_key(&new_dir));
    assert_eq!(usage.directories[&new_dir].num_accesses, 1, "Incorrect number of accesses");
    assert!(usage.directories[&new_dir].score > 0.1, "Score not greater than zero");
    assert!(usage.directories[&new_dir].last_accessed > 1, "Last accessed time not greater than zero");
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

    let usage = store::read_store(&store_file.to_path_buf());
    println!("{:?}", absolute_dir);

    assert!(usage.directories.contains_key(&absolute_dir));
  }
}
