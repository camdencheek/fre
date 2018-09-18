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


    let exists = predicates::str::contains(dir).from_utf8();

    Command::main_binary()
      .unwrap()
      .arg("--store")
      .arg(&store_file.as_os_str())
      .arg("--sorted")
      .assert()
      .stdout(exists);

    // TODO figure out how to assert increase

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

    let exists = predicates::str::contains(new_dir).from_utf8();

    Command::main_binary()
      .unwrap()
      .arg("--store")
      .arg(&store_file.as_os_str())
      .arg("--sorted")
      .assert()
      .stdout(exists);

    // TODO figure out how to assert increase
  }

  #[test]
  fn add_relative() {
    let store_file = common::get_tempfile_path();
    let relative_dir = "/home/test/../random_relative_dir".to_string();
    let absolute_dir = "/home/random_relative_dir".to_string();

    Command::main_binary()
      .unwrap()
      .current_dir(std::env::temp_dir().as_os_str())
      .arg("--store")
      .arg(&store_file.as_os_str())
      .arg("--add")
      .arg(&relative_dir)
      .assert();

    let relative = predicates::str::contains(absolute_dir).from_utf8();

    Command::main_binary()
      .unwrap()
      .current_dir(std::env::temp_dir().as_os_str())
      .arg("--store")
      .arg(&store_file.as_os_str())
      .arg("--sorted")
      .assert()
      .stdout(relative);

  }
}
