use topd::{store, args, SortMethod};
use std::path::PathBuf;
use path_absolutize::*;
use std::process;
use std::str::FromStr;


fn main() {
    let matches = args::parse_args();

    let store_file = matches.value_of("store")
      .map(|s| {
        PathBuf::from(s).absolutize()
          .expect(&format!("Unable to get absolute path of {}", s))
      }).unwrap_or_else(|| topd::default_store_path()); // using unwrap_or_else instead of unwrap_or because lazily evaluated

    let mut usage = match store::read_store(&store_file) {
      Ok(u) => u,
      Err(e) => {
        eprintln!("Unable to read store file: {}", e);
        process::exit(1);
      }
    };

    if matches.is_present("purge") {
      usage.purge();
    }

    let sort_method = match matches.value_of("sort_method") {
      Some("recent") => SortMethod::Recent,
      Some("frequent") => SortMethod::Frequent,
      Some("frecent") => SortMethod::Frecent,
      None => SortMethod::Frecent,
      Some(_) => unreachable!(),
    };

    if matches.is_present("sorted") || matches.is_present("stat") {
      let limit = matches.value_of("limit")
        .map(|s| {
          s.parse::<u64>().expect(format!("invalid u64 {}", s).as_str())
        });
      usage.print_sorted(&sort_method, matches.is_present("stat"), limit);
    }

    if matches.is_present("add") {
      let dir = matches.value_of("directory")
        .unwrap();
      usage.add(
        PathBuf::from(dir).absolutize()
        .expect(&format!("Unable to get absolute path of {}", dir))
        .to_str()
        .expect("Unable to convert absolute path to string")
        .to_string()
      );
    }

    if let Some(weight) = matches.value_of("increase") {

      let weight = f64::from_str(weight).unwrap();

      let dir = matches.value_of("directory")
        .unwrap();

      let ref_time = usage.reference_time.clone();

      &usage.find_mut(
        &PathBuf::from(dir).absolutize()
        .expect(&format!("Unable to get absolute path of {}", dir))
        .to_str()
        .expect("Unable to convert absolute path to string")
        .to_string()
      ).unwrap().increase(weight, ref_time);
    }

    if let Some(weight) = matches.value_of("decrease"){

      let weight = f64::from_str(weight).unwrap();

      let dir = matches.value_of("directory")
        .unwrap();

      let ref_time = usage.reference_time.clone();
      &usage.find_mut(
        &PathBuf::from(dir).absolutize()
        .expect(&format!("Unable to get absolute path of {}", dir))
        .to_str()
        .expect("Unable to convert absolute path to string")
        .to_string()
      ).unwrap().decrease(weight, ref_time);
    }

    if let Some(n) = matches.value_of("truncate")  {
      let keep_num = n.parse::<usize>().expect(&format!("invalid usize {}", n));
      usage.truncate(keep_num, &sort_method);
    }

    if let Err(e) = store::write_store(&usage, &store_file) {
      eprintln!("Unable to write to store file: {}", e);
      process::exit(2);
    }
}
