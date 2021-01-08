use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn version_long() {
    let expected_version = predicate::str::similar(format!(
        "{} {}\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    ))
    .from_utf8();

    Command::main_binary()
        .unwrap()
        .arg("--version")
        .assert()
        .stdout(expected_version);
}

#[test]
fn version_short() {
    let expected_version = predicate::str::similar(format!(
        "{} {}\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    ))
    .from_utf8();

    Command::main_binary()
        .unwrap()
        .arg("-V")
        .assert()
        .stdout(expected_version);
}
