use super::common;
use std::process::Command;
use predicates::prelude::*;
use assert_cmd::prelude::*;

#[test]
fn add_existing_exists() {
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
}

#[test]
fn add_existing_increases() {
  let store_file = common::get_tempfile_path();
  let dir = "/home".to_string();

  Command::main_binary()
    .unwrap()
    .arg("--store")
    .arg(&store_file.as_os_str())
    .arg("--add")
    .arg(&dir)
    .assert();


  let increased = common::path_score_approx_equal(dir, 3.0); 

  Command::main_binary()
    .unwrap()
    .arg("--store")
    .arg(&store_file.as_os_str())
    .arg("--stat")
    .arg("--sort_method")
    .arg("frequent")
    .assert()
    .stdout(increased);
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
    .assert()
    .success();

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
    .assert()
    .success();

  let absolute_created_correctly = predicates::str::contains(absolute_dir).from_utf8();

  Command::main_binary()
    .unwrap()
    .current_dir(std::env::temp_dir().as_os_str())
    .arg("--store")
    .arg(&store_file.as_os_str())
    .arg("--sorted")
    .assert()
    .stdout(absolute_created_correctly);

}


#[test]
fn increase_accesses() {
  let store_file = common::get_tempfile_path();
  let absolute_dir = "/home".to_string();

  Command::main_binary()
    .unwrap()
    .arg("--store")
    .arg(&store_file.as_os_str())
    .arg("--increase")
    .arg("2.0")
    .arg(&absolute_dir)
    .assert()
    .success();

  let accesses_increased_two = common::path_score_approx_equal(absolute_dir.clone(), 4.0);

  Command::main_binary()
    .unwrap()
    .arg("--store")
    .arg(&store_file.as_os_str())
    .arg("--stat")
    .arg("--sort_method")
    .arg("frequent")
    .assert()
    .stdout(accesses_increased_two);
}

#[test]
fn decrease_accesses() {
  let store_file = common::get_tempfile_path();
  let absolute_dir = "/home".to_string();

  Command::main_binary()
    .unwrap()
    .current_dir(std::env::temp_dir().as_os_str())
    .arg("--store")
    .arg(&store_file.as_os_str())
    .arg("--decrease")
    .arg("1.0")
    .arg(&absolute_dir)
    .assert()
    .success();

  let accesses_decreased_one = common::path_score_approx_equal(absolute_dir.clone(), 1.0);

  Command::main_binary()
    .unwrap()
    .current_dir(std::env::temp_dir().as_os_str())
    .arg("--store")
    .arg(&store_file.as_os_str())
    .arg("--stat")
    .arg("--sort_method")
    .arg("frequent")
    .assert()
    .stdout(accesses_decreased_one);
}


#[test]
fn increase_score() {
  let store_file = common::get_tempfile_path();
  let absolute_dir = "/home".to_string();

  Command::main_binary()
    .unwrap()
    .current_dir(std::env::temp_dir().as_os_str())
    .arg("--store")
    .arg(&store_file.as_os_str())
    .arg("--increase")
    .arg("2.0")
    .arg(&absolute_dir)
    .assert()
    .success();

  let frecency_increased_two = common::path_score_increased(absolute_dir.clone(), 3.1);

  Command::main_binary()
    .unwrap()
    .current_dir(std::env::temp_dir().as_os_str())
    .arg("--store")
    .arg(&store_file.as_os_str())
    .arg("--stat")
    .arg("--sort_method")
    .arg("frecent")
    .assert()
    .stdout(frecency_increased_two);

}

#[test]
fn decrease_score() {
  let store_file = common::get_tempfile_path();
  let absolute_dir = "/home".to_string();

  Command::main_binary()
    .unwrap()
    .current_dir(std::env::temp_dir().as_os_str())
    .arg("--store")
    .arg(&store_file.as_os_str())
    .arg("--decrease")
    .arg("1.0")
    .arg(&absolute_dir)
    .assert()
    .success();

  let frecency_decreased_one = common::path_score_decreased(absolute_dir.clone(), 2.9);

  Command::main_binary()
    .unwrap()
    .current_dir(std::env::temp_dir().as_os_str())
    .arg("--store")
    .arg(&store_file.as_os_str())
    .arg("--stat")
    .arg("--sort_method")
    .arg("frecent")
    .assert()
    .stdout(frecency_decreased_one);

}
