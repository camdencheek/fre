mod serialize;

use super::stats::PathStats;
use super::current_time_secs;
use super::SortMethod;
use std::default::Default;
use std::io::{self, BufWriter, BufReader, Write};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process;
use log::error;

/// Parses the file at `path` into a `UsageStore` object
pub fn read_store(path: &PathBuf) -> Result<UsageStore, io::Error> {
    if path.is_file() {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let store: serialize::UsageStoreSerializer = serde_json::from_reader(reader)?;
        Ok(UsageStore::from(store))
    } else {
        Ok(UsageStore::default())
    }
}

/// Serializes and writes a `UsageStore` to a file
pub fn write_store(store: UsageStore, path: &PathBuf) -> io::Result<()> {
    let store_dir = path.parent().expect("file must have parent");
    fs::create_dir_all(&store_dir)?;
    let file = File::create(&path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &serialize::UsageStoreSerializer::from(store))?;

    Ok(())
}

/// A collection of statistics about paths
pub struct UsageStore {
    reference_time: f64,
    half_life: f32,
    pub paths: Vec<PathStats>,
}

impl Default for UsageStore {
    fn default() -> UsageStore {
        UsageStore {
            reference_time: current_time_secs(),
            half_life: 60.0 * 60.0 * 24.0 * 3.0, // three day half life
            paths: Vec::new(),
        }
    }
}

impl UsageStore {
    /// Remove all paths from the store that do not exist
    pub fn purge(&mut self) {
        self.paths.retain(|dir| Path::new(&dir.path).exists());
    }

    /// Remove all but the top N (sorted by `sort_method`) from the `UsageStore`
    pub fn truncate(&mut self, keep_num: usize, sort_method: &SortMethod) {
        let mut sorted_vec = self.sorted(sort_method);
        sorted_vec.truncate(keep_num);
        self.paths = sorted_vec;
    }

    /// Reset the reference time to now, and reweight all the statistics to reflect that
    pub fn reset_time(&mut self) {
        let current_time = current_time_secs();

        self.reference_time = current_time;

        for path in self.paths.iter_mut() {
            path.reset_ref_time(current_time);
        }
    }

    /// Change the half life and reweight such that frecency does not change
    pub fn set_half_life(&mut self, half_life: f32) {
        self.reset_time();
        self.half_life = half_life;

        for path in self.paths.iter_mut() {
            path.set_half_life(half_life);
        }
    }

    /// Log a visit to a path
    pub fn add(&mut self, path: &str) {
        let path_stats = self.get(&path);

        path_stats.update_frecency(1.0);
        path_stats.update_num_accesses(1);
        path_stats.update_last_access(current_time_secs());
    }

    /// Adjust the score of a path by a given weight
    pub fn adjust(&mut self, path: &str, weight: f32) {
        let path_stats = self.get(&path);

        path_stats.update_frecency(weight);
        path_stats.update_num_accesses(weight as i32);
    }


    /// Print out all the paths, sorted by `method`, with an optional maximum of `limit`
    pub fn print_sorted(&self, method: &SortMethod, show_stats: bool, limit: Option<usize>) {
        let stdout = io::stdout();
        let handle = stdout.lock();
        let mut writer = BufWriter::new(handle);

        let sorted = self.sorted(method);
        let take_num = limit.unwrap_or_else(|| sorted.len());

        for dir in sorted.iter().take(take_num) {
            writer.write_all(dir.to_string(method, show_stats).as_bytes())
                .unwrap_or_else(|e| {
                    error!("unable to write to stdout: {}", e);
                    process::exit(1);
                });
        }
    }

    /// Return a sorted vector of all the paths in the store, sorted by `sort_method`
    fn sorted(&self, sort_method: &SortMethod) -> Vec<PathStats> {
        let mut new_vec = self.paths.clone();
        new_vec.sort_by(|dir1, dir2| {
            dir1.cmp_score(dir2, sort_method).reverse()
        });

        new_vec
    }

    /// Retrieve a mutable reference to a path in the store.
    /// If the path does not exist, create it and return a reference to the created path
    fn get(&mut self, path: &str) -> &mut PathStats {
        match self.paths.binary_search_by_key(&path, |dir_stats| &dir_stats.path) {
            Ok(idx) => &mut self.paths[idx],
            Err(idx) => {
                self.paths.insert(
                    idx,
                    PathStats::new(path, self.reference_time, self.half_life),
                );
                &mut self.paths[idx]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    fn create_usage() -> UsageStore {
        UsageStore {
            reference_time: current_time_secs(),
            half_life: 1.0,
            paths: Vec::new(),
        }
    }

    #[test]
    fn add_new() {
        let mut usage = create_usage();

        usage.add("test");

        assert_that!(usage.paths.len()).is_equal_to(1);
    }

    #[test]
    fn add_existing() {
        let mut usage = create_usage();

        usage.add("test");
        usage.add("test");

        assert_that!(usage.paths.len()).is_equal_to(1);
    }

    #[test]
    fn adjust_existing() {
        let mut usage = create_usage();

        usage.add("test");
        usage.adjust("test", 3.0);

        assert_that!(usage.paths.len()).is_equal_to(1);
    }

    #[test]
    fn adjust_new() {
        let mut usage = create_usage();

        usage.adjust("test", 3.0);

        assert_that!(usage.paths.len()).is_equal_to(1);
    }


    #[test]
    fn purge_exists() {
        let mut usage = create_usage();
        let file = tempfile::NamedTempFile::new().unwrap().into_temp_path();
        let path = file
            .as_os_str()
            .to_str()
            .unwrap();
        usage.add(path);

        usage.purge();
        assert_that!(usage.paths.len()).is_equal_to(1);
    }

    #[test]
    fn purge_not_exists() {
        let mut usage = create_usage();
        usage.add("/nonexistant_dir");

        usage.purge();
        assert_that!(usage.paths.len()).is_equal_to(0);
    }

    #[test]
    fn truncate_greater() {
        let mut usage = create_usage();
        usage.add("dir1");
        usage.add("dir2");

        usage.truncate(1, &SortMethod::Recent);

        assert_that!(usage.paths.len()).is_equal_to(1);
    }

    #[test]
    fn truncate_less() {
        let mut usage = create_usage();
        usage.add("dir1");
        usage.add("dir2");

        usage.truncate(3, &SortMethod::Recent);

        assert_that!(usage.paths.len()).is_equal_to(2);
    }

    #[test]
    fn sorted_frecent() {
        let mut usage = create_usage();
        usage.add("dir1");
        usage.add("dir2");
        usage.get("dir2").update_frecency(1000.0);

        let sorted = usage.sorted(&SortMethod::Frecent);

        assert_that!(sorted.len()).is_equal_to(2);
        assert_that!(sorted[0].path).is_equal_to("dir2".to_string());
    }

    #[test]
    fn sorted_recent() {
        let mut usage = create_usage();
        usage.add("dir1");
        usage.add("dir2");
        usage.get("dir2").update_last_access(current_time_secs() + 100.0);

        let sorted = usage.sorted(&SortMethod::Recent);

        assert_that!(sorted.len()).is_equal_to(2);
        assert_that!(sorted[0].path).is_equal_to("dir2".to_string());
    }

    #[test]
    fn sorted_frequent() {
        let mut usage = create_usage();
        usage.add("dir1");
        usage.add("dir2");
        usage.get("dir2").update_num_accesses(100);

        let sorted = usage.sorted(&SortMethod::Frequent);

        assert_that!(sorted.len()).is_equal_to(2);
        assert_that!(sorted[0].path).is_equal_to("dir2".to_string());
    }

    #[test]
    fn get_exists() {
        let mut usage = create_usage();
        usage.add("dir1");

        let _stats = usage.get("dir1");

        assert_that!(usage.paths.len()).is_equal_to(1);
    }

    #[test]
    fn get_not_exists() {
        let mut usage = create_usage();
        usage.add("dir1");

        usage.get("dir2");

        assert_that!(usage.paths.len()).is_equal_to(2);
    }

    #[test]
    fn reset_time() {
        let mut usage = create_usage();
        let current_time = current_time_secs();
        usage.reference_time = current_time - 10.0;
        usage.add("test");
        let original_frecency = usage.get("test").get_frecency();

        usage.reset_time();

        assert_that!(usage.reference_time).is_close_to(current_time, 0.1);
        assert_that!(usage.get("test").get_frecency()).is_close_to(original_frecency, 0.1)
    }

    #[test]
    fn set_halflife() {
        let mut usage = create_usage();
        let current_time = current_time_secs();
        usage.reference_time = current_time - 10.0;
        usage.add("dir1");
        let original_frecency = usage.get("dir1").get_frecency();
        usage.set_half_life(10.0);

        let new_frecency = usage.get("dir1").get_frecency();

        assert_that!(usage.half_life).is_close_to(10.0, 0.01);
        assert_that!(new_frecency).is_close_to(original_frecency, 0.01);
    }
}
