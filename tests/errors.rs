#![feature(box_syntax)]
mod common;

mod errors {
  use super::common;
  use assert_cmd::prelude::*;
  use std::process::Command;
  use predicates::prelude::*;
  use topd::store;
  use std::path::PathBuf;
  use tempfile;

  #[test]
  fn invalid_store() {

    let empty = predicates::str::is_empty().from_utf8();
    let error = predicates::str::contains("Unable to read store file").from_utf8();
    let mut file = tempfile::NamedTempFile::new().unwrap();

    Command::main_binary()
      .unwrap()
      .arg("--store")
      .arg(file.path().as_os_str())
      .arg("--sorted")
      .assert()
      .code(1)
      .stdout(empty)
      .stderr(error);
  }

  #[test]
  fn non_writable() {

    let empty = predicates::str::is_empty().from_utf8();
    let error = predicates::str::contains("Permission denied").from_utf8();
    let mut file = tempfile::NamedTempFile::new().unwrap();

    Command::main_binary()
      .unwrap()
      .arg("--store")
      .arg("/testdir")
      .arg("--sorted")
      .assert()
      .code(2)
      .stdout(empty)
      .stderr(error);
  }

}
