use path_absolutize::*;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;
use topd::{args, store, SortMethod};

fn main() {
    let matches = args::get_app().get_matches();

    let store_file = args::get_store_path(&matches); 

    let mut usage = store::read_store(&store_file).unwrap_or_else(|e| {
        eprintln!("Unable to read store file: {}", e);
        process::exit(1);
    });

    if matches.is_present("purge") {
        usage.purge();
    }

    let sort_method = match matches.value_of("sort_method") {
        Some("recent") => SortMethod::Recent,
        Some("frequent") => SortMethod::Frequent,
        Some("frecent") => SortMethod::Frecent,
        None => SortMethod::Frecent,
        Some(_) => unreachable!(), // enforced by clap
    };

    if matches.is_present("sorted") || matches.is_present("stat") {
        let limit = matches.value_of("limit").map(|s| {
            s.parse::<usize>().unwrap_or_else(|_| {
                eprintln!("Invalid result limit {}", s);
                process::exit(1);
            })
        });

        usage.print_sorted(&sort_method, matches.is_present("stat"), limit);
    }

    if matches.is_present("add") {
        // This unwrap is okay because clap should catch a missing
        // directory before this
        let dir = matches.value_of("directory").unwrap();

        let path = match PathBuf::from(dir).absolutize() {
            Err(e) => {
                eprintln!("Unable to get absolute path of {}: {}", dir, e);
                process::exit(1);
            }
            Ok(p) => p
                .to_str()
                .unwrap_or_else(|| {
                    eprintln!("Unable to convert absolute path {:?} to string", p);
                    process::exit(1);
                })
                .to_string(),
        };

        usage.add(&path);
    }

    if matches.is_present("increase") || matches.is_present("decrease") {
        let weight = match (matches.value_of("increase"), matches.value_of("decrease")) {
            (Some(i), None) => f64::from_str(i).unwrap_or_else(|_| {
                eprintln!("Unable to parse weight from {}", i);
                process::exit(1);
            }),
            (None, Some(d)) => -f64::from_str(d).unwrap_or_else(|_| {
                eprintln!("Unable to parse weight from {}", d);
                process::exit(1);
            }),
            _ => unreachable!(),
        };

        let input_path = matches.value_of("directory").unwrap(); // enforced by clap

        let absolute_path = PathBuf::from(input_path)
            .absolutize()
            .unwrap_or_else(|_| {
                eprintln!("Unable to get absolute path of {}", input_path);
                process::exit(1)
            })
            .to_str()
            .unwrap_or_else(|| {
                eprintln!("Unable to convert absolute_path path to string");
                process::exit(1)
            })
            .to_string();

        usage.adjust(&absolute_path, weight); 
    }

    if let Some(n) = matches.value_of("truncate") {
        match n.parse::<usize>() {
            Ok(keep_num) => usage.truncate(keep_num, &sort_method),
            Err(_) => {
                eprintln!("Invalid truncate limit {}", n);
                process::exit(1);
            }
        }
    }


    if let Err(e) = store::write_store(&usage, &store_file) {
        eprintln!("Unable to write to store file: {}", e);
        process::exit(2);
    }
}



