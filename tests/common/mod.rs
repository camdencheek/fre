use log::error;
use predicates::*;
use std::collections::HashMap;
use std::io::Write;
use std::process;
use std::str;
use std::time::SystemTime;
use tempfile;

pub fn get_tempfile_path() -> tempfile::TempPath {
    let mut file = tempfile::NamedTempFile::new().unwrap();

    let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => (n.as_secs() as u128 * 1000 + n.subsec_millis() as u128) as f64 / 1000.0,
        Err(e) => {
            error!("invalid system time: {}", e);
            process::exit(1);
        }
    };

    file.write(
        format!(
            r#"{{
      "reference_time": {},
      "half_life": 259200.0,
      "items": [
        {{
          "item": "/home",
          "frecency": 3.0,
          "last_accessed": -100.0,
          "num_accesses": 2
        }},
        {{
          "item": "/home/nonexistant_dir",
          "frecency": 2.0,
          "last_accessed": 1.0,
          "num_accesses": 1
        }},
        {{
          "item": "/",
          "frecency": 1.0,
          "last_accessed": 0.0,
          "num_accesses": 3
        }}
      ]
    }}"#,
            current_time
        )
        .as_bytes(),
    )
    .unwrap();

    return file.into_temp_path();
}

pub fn parse_scored_output(output: &str) -> Option<HashMap<String, f64>> {
    use std::f64;

    let mut out_map = HashMap::new();
    for line in output.lines() {
        let mut elems = line.split_whitespace();
        let score: f64 = elems
            .next()
            .expect("no score on this line")
            .parse::<f64>()
            .unwrap();
        let item = elems.next().expect("no item on this line");
        out_map.insert(item.to_string(), score);
    }

    return Some(out_map);
}

pub fn item_score_approx_equal(item: String, expected: f64) -> impl Predicate<[u8]> {
    predicates::function::function(move |x: &[u8]| {
        let map = parse_scored_output(str::from_utf8(x).expect("failed to parse utf8"));
        let out_score = map
            .expect("failed to parse scored output")
            .get(&item.clone())
            .expect("item doesn't exist in output")
            .clone();
        out_score >= expected * 0.95 && out_score <= expected * 1.05
    })
}

pub fn n_results(n: usize) -> impl Predicate<[u8]> {
    predicates::function::function(move |x: &[u8]| str::from_utf8(x).unwrap().lines().count() == n)
}
