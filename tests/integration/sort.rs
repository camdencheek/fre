use super::common;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use std::str;

#[test]
fn sorted_stats() {
    let store_file = common::get_tempfile_path();

    let expected_sorted =
        predicate::str::similar("3\t/\n2\t/home\n1\t/home/nonexistant_dir\n").from_utf8();

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

    let expected_sorted = predicate::str::similar("/home\n/home/nonexistant_dir\n/\n").from_utf8();

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

    let expected_sorted = predicate::str::similar("/home/nonexistant_dir\n/\n/home\n").from_utf8();

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

    let expected_sorted = predicate::str::similar("/\n/home\n/home/nonexistant_dir\n").from_utf8();

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

    let expected_error = predicate::str::contains("'badsort' isn't a valid value").from_utf8();

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

#[test]
fn truncate() {
    let store_file = common::get_tempfile_path();

    Command::main_binary()
        .unwrap()
        .arg("--store")
        .arg(&store_file.as_os_str())
        .arg("--truncate")
        .arg("2")
        .assert()
        .success();

    let two_lines = predicate::function(|x: &[u8]| str::from_utf8(x).unwrap().lines().count() == 2);

    Command::main_binary()
        .unwrap()
        .arg("--store")
        .arg(&store_file.as_os_str())
        .arg("--stat")
        .assert()
        .stdout(two_lines);
}

#[test]
fn limit() {
    let store_file = common::get_tempfile_path();

    let two_lines = common::n_results(2);

    Command::main_binary()
        .unwrap()
        .arg("--store")
        .arg(&store_file.as_os_str())
        .arg("--sorted")
        .arg("--limit")
        .arg("2")
        .assert()
        .success()
        .stdout(two_lines);
}

#[test]
fn limit_too_many() {
    let store_file = common::get_tempfile_path();

    let three_lines = common::n_results(3);

    Command::main_binary()
        .unwrap()
        .arg("--store")
        .arg(&store_file.as_os_str())
        .arg("--sorted")
        .arg("--limit")
        .arg("4")
        .assert()
        .success()
        .stdout(three_lines);
}

#[test]
fn change_half_life_maintain_frecency() {
    let store_file = common::get_tempfile_path();

    Command::main_binary()
        .unwrap()
        .arg("--store")
        .arg(&store_file.as_os_str())
        .arg("--halflife")
        .arg("1000")
        .assert()
        .success();

    let score_same = common::item_score_approx_equal("/".to_string(), 1.0);

    Command::main_binary()
        .unwrap()
        .arg("--store")
        .arg(&store_file.as_os_str())
        .arg("--stat")
        .assert()
        .stdout(score_same);
}

#[test]
fn change_half_life_new_decay() {
    let store_file = common::get_tempfile_path();

    Command::main_binary()
        .unwrap()
        .arg("--store")
        .arg(&store_file.as_os_str())
        .arg("--halflife")
        .arg("100.0")
        .assert()
        .success();

    let score_half = common::item_score_approx_equal("/home".to_string(), 3.0);

    Command::main_binary()
        .unwrap()
        .arg("--store")
        .arg(&store_file.as_os_str())
        .arg("--stat")
        .assert()
        .stdout(score_half);
}
