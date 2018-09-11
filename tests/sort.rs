mod common;


mod sort {

  use super::common;
  use assert_cmd::prelude::*;
  use std::process::Command;
  use predicates::prelude::*;

  #[test]
  fn sorted_stats() {
    let store_file = common::get_tempfile_path();

    let expected_sorted = predicate::str::similar("3 /\n2 /home\n1 /home/nonexistant_dir\n")
      .from_utf8();

    Command::main_binary()
      .unwrap()
      .arg("--store")
      .arg(&store_file.as_os_str())
      .arg("--stat")
      .arg("--sort_method")
      .arg("frequent")
      .assert()
      .stdout(expected_sorted);
  }

  #[test]
  fn sorted_frecent() {
    let store_file = common::get_tempfile_path();

    let expected_sorted = predicate::str::similar("/home\n/home/nonexistant_dir\n/\n")
      .from_utf8();

    Command::main_binary()
      .unwrap()
      .arg("--store")
      .arg(&store_file.as_os_str())
      .arg("--sorted")
      .assert()
      .stdout(expected_sorted);
  }

  #[test]
  fn sorted_recent() {
    let store_file = common::get_tempfile_path();

    let expected_sorted = predicate::str::similar("/home/nonexistant_dir\n/\n/home\n")
      .from_utf8();

    Command::main_binary()
      .unwrap()
      .arg("--store")
      .arg(&store_file.as_os_str())
      .arg("--sorted")
      .arg("--sort_method")
      .arg("recent")
      .assert()
      .stdout(expected_sorted);
  }

  #[test]
  fn sorted_frequent() {
    let store_file = common::get_tempfile_path();

    let expected_sorted = predicate::str::similar("/\n/home\n/home/nonexistant_dir\n")
      .from_utf8();

    Command::main_binary()
      .unwrap()
      .arg("--store")
      .arg(&store_file.as_os_str())
      .arg("--sorted")
      .arg("--sort_method")
      .arg("frequent")
      .assert()
      .stdout(expected_sorted);
  }

  #[test]
  fn sorted_invalid() {
    let store_file = common::get_tempfile_path();

    let expected_error = predicate::str::contains("'badsort' isn't a valid value")
      .from_utf8();

    Command::main_binary()
      .unwrap()
      .arg("--store")
      .arg(&store_file.as_os_str())
      .arg("--sorted")
      .arg("--sort_method")
      .arg("badsort")
      .assert()
      .stderr(expected_error);
  }
}

