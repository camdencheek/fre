use log::error;
use env_logger;
use path_absolutize::*;
use std::path::PathBuf;
use std::str::FromStr;
use fret::*;

fn main() {

    // Set up the logger
    env_logger::Builder::from_default_env()
        .default_format_timestamp(false)
        .default_format_module_path(false)
        .init();


    let matches = args::get_app().get_matches();

    // Construct the path to the store file
    let store_file = args::get_store_path(&matches);

    // Attempt to read and unmarshal the store file
    let mut usage = match store::read_store(&store_file) {
        Ok(u) => u,
        Err(e) => error_and_exit!("unable to read store file {:?}: {}", &store_file, e)
    };


    // If a new half life is defined, parse and set it
    if let Some(h) = matches.value_of("halflife") {
        let half_life = match h.parse::<f32>() {
            Ok(h) => h,
            Err(_) => error_and_exit!("invalid half life '{}'", h)
        };

        usage.set_half_life(half_life);
    }

    // TODO write a test for this
    if usage.half_lives_passed() > 5.0 {
        usage.reset_time()
    }

    // Determine the sorting method. Defaults to frecent if unspecified
    let sort_method = match matches.value_of("sort_method") {
        Some("recent") => SortMethod::Recent,
        Some("frequent") => SortMethod::Frequent,
        Some("frecent") => SortMethod::Frecent,
        None => SortMethod::Frecent,
        Some(_) => unreachable!(), // enforced by clap
    };

    // Print the directories if --sorted or --stat are specified
    if matches.is_present("sorted") || matches.is_present("stat") {
        // If a limit is specified, parse it and use it
        if let Some(s) = matches.value_of("limit") {
            match s.parse::<usize>() {
                Ok(l) => usage.print_sorted(&sort_method, matches.is_present("stat"), Some(l)),
                Err(_) => error_and_exit!("invalid limit '{}'", s)
            };
        } else {
            usage.print_sorted(&sort_method, matches.is_present("stat"), None);
        }
    }

    // Increment a directory
    if matches.is_present("add") {
        // This unwrap is okay because clap should catch a missing directory before this
        let dir = matches.value_of("path").unwrap();

        // Convert any relative paths to absolute
        let absolute_path = match PathBuf::from(dir).absolutize() {
            Ok(p) => p.to_string_lossy().into_owned(),
            Err(e) => error_and_exit!("unable to get absolute path of {}: {}", dir, e),
        };

        usage.add(&absolute_path);
    }


    // Handle increasing or decreasing a directory's score by a given weight
    if matches.is_present("increase") || matches.is_present("decrease") {

        // Get a positive weight if increase, negative if decrease
        let weight = match (matches.value_of("increase"), matches.value_of("decrease")) {
            (Some(i), None) => f32::from_str(i).unwrap_or_else(|_| {
                error_and_exit!("invalid weight '{}'", i)
            }),
            (None, Some(d)) => -1.0 * f32::from_str(d).unwrap_or_else(|_| {
                error_and_exit!("unable to parse weight from {}", d);
            }),
            _ => unreachable!(), // enforced by clap and block guard
        };

        // Get the path to increase/decrease
        let input_path = matches.value_of("path").unwrap(); // enforced by clap

        let absolute_path = match PathBuf::from(input_path).absolutize() {
            Ok(path) => path.to_string_lossy().into_owned(),
            Err(e) => error_and_exit!("unable to convert absolute path to string: {}", e)
        };

        usage.adjust(&absolute_path, weight);
    }

    // Remove non-existant directories from store
    if matches.is_present("purge") {
        usage.purge();
    }

    // Truncate store to top N directories
    if let Some(n) = matches.value_of("truncate") {
        match n.parse::<usize>() {
            Ok(keep_num) => usage.truncate(keep_num, &sort_method),
            Err(_) => error_and_exit!("invalid truncate limit '{}'", n)
        }
    }


    // Write the updated store file
    if let Err(e) = store::write_store(usage, &store_file) {
        error_and_exit!("unable to write to store file: {}", e);
    }
}



