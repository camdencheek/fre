use std::io::Write;
use tempfile;
use std::collections::HashMap;
use predicates::{self,Predicate};
use std::str;


pub fn get_tempfile_path() -> tempfile::TempPath {

    let mut file = tempfile::NamedTempFile::new().unwrap();


    file.write(r#"{
      "reference_time": "2018-09-16T17:56:35.402314544",
      "half_life":1209600,
      "paths": [
        {
          "half_life":1209600,
          "reference_time": "2018-09-16T17:56:35.402314544",
          "path": "/home",
          "frecency": 3.0,
          "last_accessed":10,
          "num_accesses": 2
        },
        {
          "half_life":1209600,
          "reference_time": "2018-09-16T17:56:35.402314544",
          "path": "/home/nonexistant_dir",
          "frecency": 2.0,
          "last_accessed":30,
          "num_accesses": 1
        },
        {
          "half_life":1209600,
          "reference_time": "2018-09-16T17:56:35.402314544",
          "path": "/",
          "frecency": 1.0,
          "last_accessed":20,
          "num_accesses": 3
        }
      ]
    }"#
        .as_bytes()).unwrap();

    return file.into_temp_path()
}


/*
 *fn random_usage(n: u64) -> topd::store::UsageStore {
 *    let mut usage = store::UsageStore::default();
 *    let ref_time = SystemTime::now();
 *    for i in 0..n {
 *      let dir = format!("/home/ccheek/test/test{}", i).to_string();
 *      usage.add(dir.clone());
 *      let new_dir = usage.find_mut(&dir).unwrap();
 *
 *      new_dir.last_accessed = random();
 *      new_dir.frecency = random();
 *      new_dir.num_accesses = random();
 *    }
 *
 *    return usage
 *}
 */

pub fn parse_scored_output(output: &str) -> Option<HashMap<String, f64>> {
  use std::f64;

  let mut out_map = HashMap::new();
  for line in output.lines()  {
    let mut elems = line.split_whitespace();
    let score: f64 = elems.next().unwrap().parse::<f64>().unwrap();
    let path = elems.next().unwrap();
    out_map.insert(path.to_string(), score);
  }

  return Some(out_map)
}

pub fn parse_list_output(output: &str) -> Option<Vec<String>> {
  let mut out_vec = Vec::new();
  for line in output.lines()  {
    let mut elems = line.split_whitespace();
    let path = elems.next().unwrap();
    out_vec.push(path.to_string());
  }

  return Some(out_vec)
}

pub fn path_score_approx_equal(path: String, expected: f64) -> impl Predicate<[u8]> {
    predicates::function::function(move |x: &[u8]| {
      let map = parse_scored_output(str::from_utf8(x).unwrap());
      let out_score = map.unwrap().get(&path.clone()).unwrap().clone();
      
      out_score >= expected * 0.95 && out_score <= expected * 1.05
    })
}

pub fn path_score_increased(path: String, expected: f64) -> impl Predicate<[u8]> {
    predicates::function::function(move |x: &[u8]| {
      let map = parse_scored_output(str::from_utf8(x).unwrap());
      let out_score = map.unwrap().get(&path.clone()).unwrap().clone();
      
      out_score >= expected * 0.95 
    })
}

pub fn path_score_decreased(path: String, expected: f64) -> impl Predicate<[u8]> {
    predicates::function::function(move |x: &[u8]| {
      let map = parse_scored_output(str::from_utf8(x).unwrap());
      let out_score = map.unwrap().get(&path.clone()).unwrap().clone();
      
      out_score <= expected * 0.95 
    })
}

pub fn n_results(n: usize) -> impl Predicate<[u8]> {
    predicates::function::function(move |x: &[u8]| {
      str::from_utf8(x).unwrap().lines().count() == n
    })
}

