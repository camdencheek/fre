use topd::{store, args, SortMethod, stats};
use std::path::PathBuf;
use path_absolutize::*;
use rand::prelude::*;


fn main() {
    let matches = args::parse_args();

    let store_file = matches.value_of("store")
      .map(|s| PathBuf::from(s).absolutize()
           .expect(&format!("Unable to get absolute path of {}", s)))
      // using unwrap_or_else instead of unwrap_or because lazily evaluated
      .unwrap_or_else(|| topd::default_store_path()); 


    let mut usage = store::read_store(&store_file);

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

    store::write_store(&usage, &store_file);
}
