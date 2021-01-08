use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile;

#[test]
fn invalid_store() {
    let empty = predicates::str::is_empty().from_utf8();
    let error = predicates::str::contains("unable to read store file").from_utf8();
    let file = tempfile::NamedTempFile::new().unwrap();

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
    let error = predicates::str::is_empty().from_utf8().not();

    Command::main_binary()
        .unwrap()
        .arg("--store")
        .arg("/testdir")
        .arg("--sorted")
        .assert()
        .code(1)
        .stdout(empty)
        .stderr(error);
}
