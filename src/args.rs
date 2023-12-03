use anyhow::{anyhow, Result};
use clap::{builder::OsStr, Args, Parser, ValueEnum};
use directories::ProjectDirs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Use a non-default store file
    #[arg(long = "store_name", conflicts_with = "store")]
    pub store_name: Option<PathBuf>,

    /// Use a non-default filename for the store file in the default store directory
    #[arg(long, conflicts_with = "store_name")]
    pub store: Option<PathBuf>,

    #[command(flatten)]
    pub updates: UpdateArgs,

    #[command(flatten)]
    pub stats: StatsArgs,

    /// The method to sort output by
    #[arg(long="sort_method", value_enum, default_value = SortMethod::Frecent)]
    pub sort_method: SortMethod,

    #[command(flatten)]
    pub janitor: JanitorArgs,

    /// The item to update
    pub item: Option<String>,
}

#[derive(Args, Debug)]
#[group(multiple = false, conflicts_with = "StatsArgs")]
pub struct UpdateArgs {
    /// Add a visit to ITEM to the store
    #[arg(short = 'a', long)]
    pub add: bool,

    /// Increase the weight of an item by WEIGHT
    #[arg(short = 'i', long, value_name = "WEIGHT")]
    pub increase: Option<f64>,

    /// Delete ITEM from the store
    #[arg(short = 'D', long)]
    pub delete: bool,

    /// Decrease the weight of a path by WEIGHT
    #[arg(short = 'd', long)]
    pub decrease: Option<f64>,
}

#[derive(Args, Debug)]
pub struct StatsArgs {
    /// Print the stored directories in order of highest to lowest score
    #[arg(long, group = "list")]
    pub sorted: bool,

    /// Print statistics about the stored directories
    #[arg(long, group = "list")]
    pub stat: bool,

    /// Limit the number of results printed with --sorted or --stat
    #[arg(long, requires = "list")]
    pub limit: Option<usize>,

    /// Override the number of digits shown with --stat
    #[arg(long, requires = "stat")]
    pub stat_digits: Option<usize>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SortMethod {
    Recent,
    Frequent,
    Frecent,
}

impl From<SortMethod> for OsStr {
    fn from(value: SortMethod) -> Self {
        match value {
            SortMethod::Recent => OsStr::from("recent"),
            SortMethod::Frequent => OsStr::from("frequent"),
            SortMethod::Frecent => OsStr::from("frecent"),
        }
    }
}

#[derive(Args, Debug)]
pub struct JanitorArgs {
    /// Change the halflife to N seconds
    #[arg(long, value_name = "N")]
    pub halflife: Option<f64>,

    /// Truncate the stored items to only the top N
    #[arg(long, short = 'T', value_name = "N")]
    pub truncate: Option<usize>,
}

/// Given the argument matches, return the path of the store file.
pub fn get_store_path(args: &Cli) -> Result<PathBuf> {
    match (&args.store, &args.store_name) {
        (Some(dir), None) => Ok(dir.to_owned()),
        (None, filename) => default_store(filename.to_owned()),
        _ => unreachable!(),
    }
}

/// Return a path to a store file in the default location.
/// Uses filename as the name of the file if it is not `None`.
pub fn default_store(filename: Option<PathBuf>) -> Result<PathBuf> {
    let store_dir = match ProjectDirs::from("", "", env!("CARGO_PKG_NAME")) {
        Some(dir) => dir.data_dir().to_path_buf(),
        None => return Err(anyhow!("failed to determine default store directory")),
    };

    let filename =
        filename.unwrap_or_else(|| PathBuf::from(format!("{}.json", env!("CARGO_PKG_NAME"))));
    let mut store_file = store_dir;
    store_file.push(filename);

    Ok(store_file.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_store_path_full() {
        let arg_vec = vec!["fre", "--store", "/test/path"];
        let args = Cli::try_parse_from(arg_vec).unwrap();

        let store_path = get_store_path(&args).unwrap();

        assert_eq!(PathBuf::from("/test/path"), store_path);
    }

    #[test]
    fn get_store_path_file() {
        let arg_vec = vec!["fre", "--store_name", "test.path"];
        let args = Cli::try_parse_from(arg_vec).unwrap();

        let store_path = get_store_path(&args).unwrap();

        assert_eq!(
            store_path
                .file_name()
                .expect("no filename on store path")
                .to_os_string()
                .to_string_lossy(),
            "test.path".to_string()
        );
    }
}
