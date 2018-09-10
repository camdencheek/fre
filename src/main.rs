use topd::{store, args, SortMethod};
use std::path::PathBuf;


fn main() {
    let matches = args::parse_args();

    let store_file = matches.value_of("store")
      .map(|s| PathBuf::from(s))
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
      Some(_) => unreachable!()
    };

    if matches.is_present("sorted") {
      let limit = matches.value_of("limit")
        .map(|s| {
          s.parse::<u64>()
            .expect(format!("invalid u64 {}", s).as_str())
        });
      usage.print_sorted(limit, &sort_method, matches.is_present("stat"));
    }

    store::write_store(&usage, &store_file);
}
