use clap::{App, AppSettings, Arg, ArgMatches};
use directories::ProjectDirs;
use log::error;
use std::path::PathBuf;
use std::process;

/// Returns Ok(_) if input string  can be parsed as an int.
/// Used for argument validation.
fn is_int(s: String) -> Result<(), String> {
    match s.parse::<i64>() {
        Ok(_) => Ok(()),
        Err(_e) => Err(format!("invalid integer {}", s)),
    }
}

/// Returns an instance of the application with arguments.
pub fn get_app() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::NextLineHelp)
        .arg(
            Arg::with_name("add")
                .short("a")
                .long("add")
                .conflicts_with_all(&["increase", "decrease", "delete", "sorted"])
                .requires("item")
                .help("Add a visit to ITEM to the store"),
        )
        .arg(
            Arg::with_name("increase")
                .short("i")
                .long("increase")
                .help("Increase the weight of an item by WEIGHT")
                .conflicts_with_all(&["add", "decrease", "delete", "sorted"])
                .requires("item")
                .value_name("WEIGHT")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("decrease")
                .short("d")
                .long("decrease")
                .conflicts_with_all(&["increase", "add", "delete", "sorted"])
                .requires("item")
                .help("Decrease the weight of a path by WEIGHT")
                .value_name("WEIGHT")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("delete")
                .short("D")
                .long("delete")
                .conflicts_with_all(&["increase", "add", "decrease", "sorted"])
                .requires("item")
                .help("Delete an item from the store"),
        )
        .arg(
            Arg::with_name("sorted")
                .long("sorted")
                .group("lists")
                .help("Print the stored directories in order of highest to lowest score"),
        )
        .arg(
            Arg::with_name("stat")
                .short("s")
                .group("lists")
                .long("stat")
                .help("Print statistics about the stored directories"),
        )
        .arg(
            Arg::with_name("sort_method")
                .long("sort_method")
                .help("The method to sort by most used")
                .takes_value(true)
                .possible_values(&["frecent", "frequent", "recent"])
                .default_value("frecent"),
        )
        .arg(
            Arg::with_name("limit")
                .long("limit")
                .short("l")
                .takes_value(true)
                .requires("lists")
                .help("Limit the number of results printed --sorted"),
        )
        .arg(
            Arg::with_name("store")
                .long("store")
                .value_name("FILE")
                .conflicts_with_all(&["store_name"])
                .help("Use a non-default store file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("store_name")
                .long("store_name")
                .value_name("FILE")
                .conflicts_with_all(&["store"])
                .help(
                    "Use a non-default filename for the store file in the default store directory",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("truncate")
                .short("T")
                .long("truncate")
                .help("Truncate the stored directories to only the top N")
                .value_name("N")
                .validator(is_int)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("halflife")
                .long("halflife")
                .help("Change the halflife to N seconds")
                .value_name("N")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("item")
                .index(1)
                .value_name("ITEM")
                .help("The item to update"),
        )
}

/// Given the argument matches, return the path of the store file.
pub fn get_store_path(matches: &ArgMatches) -> PathBuf {
    match (matches.value_of("store"), matches.value_of("store_name")) {
        (Some(dir), None) => PathBuf::from(dir),
        (None, file) => default_store(file),
        _ => unreachable!(),
    }
}

/// Return a path to a store file in the default location.
/// Uses filename as the name of the file if it is not `None`.
pub fn default_store(filename: Option<&str>) -> PathBuf {
    let store_dir = match ProjectDirs::from("", "", env!("CARGO_PKG_NAME")) {
        Some(dir) => dir.data_dir().to_path_buf(),
        None => {
            error!("Failed to detect default data directory");
            process::exit(1);
        }
    };

    let default = format!("{}.json", env!("CARGO_PKG_NAME"));
    let filename = filename.unwrap_or(&default);
    let mut store_file = store_dir;
    store_file.push(filename);

    store_file.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn get_store_path_full() {
        let arg_vec = vec!["fre", "--store", "/test/path"];
        let matches = get_app().get_matches_from_safe(arg_vec).unwrap();

        let store_path = get_store_path(&matches);

        assert_that!(store_path).is_equal_to(PathBuf::from("/test/path"));
    }

    #[test]
    fn get_store_path_file() {
        let arg_vec = vec!["fre", "--store_name", "test.path"];
        let matches = get_app().get_matches_from_safe(arg_vec).unwrap();

        let store_path = get_store_path(&matches);

        assert_that!(store_path.to_str().unwrap()).ends_with("test.path");
    }
}
