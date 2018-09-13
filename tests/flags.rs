#![feature(box_syntax)]
mod common;

mod flags {
  use super::common;
  use assert_cmd::prelude::*;
  use std::process::Command;
  use predicates::prelude::*;
  use topd::store;
  use std::path::PathBuf;

  #[test]
  fn version_long() {
    let expected_version = predicate::str::similar(format!("{} {}\n",
                                                           env!("CARGO_PKG_NAME"),
                                                           env!("CARGO_PKG_VERSION")))
      .from_utf8();

    Command::main_binary()
      .unwrap()
      .arg("--version")
      .env("stdout", "version")
      .assert()
      .stdout(expected_version);
  }


  #[test]
  fn version_short() {
    let expected_version = predicate::str::similar(format!("{} {}\n",
                                                           env!("CARGO_PKG_NAME"),
                                                           env!("CARGO_PKG_VERSION")))
      .from_utf8();

    Command::main_binary()
      .unwrap()
      .arg("-V")
      .env("stdout", "version")
      .assert()
      .stdout(expected_version);
  }


  #[test]
  fn purge() {
    let store_file = common::get_tempfile_path();

    Command::main_binary()
      .unwrap()
      .arg("--store")
      .arg(&store_file.as_os_str())
      .arg("--purge")
      .assert();

    let mut usage = store::read_store(&store_file.to_path_buf());

    println!("{:?}", usage);
    assert!(usage.find(&"/home/nonexistant_dir".to_string())
            .is_none(), 
            "Purge didn't remove /home/nonexistant_dir")
  }
}

