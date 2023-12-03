mod serialize;

use super::current_time_secs;
use super::stats::ItemStats;
use crate::args::SortMethod;
use anyhow::Result;
use std::default::Default;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::PathBuf;

/// Parses the file at `path` into a `UsageStore` object
pub fn read_store(path: &PathBuf) -> Result<FrecencyStore, io::Error> {
    if path.is_file() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let store: serialize::FrecencyStoreSerializer = serde_json::from_reader(reader)?;
        Ok(FrecencyStore::from(store))
    } else {
        Ok(FrecencyStore::default())
    }
}

/// Serializes and writes a `UsageStore` to a file
pub fn write_store(store: FrecencyStore, path: &PathBuf) -> io::Result<()> {
    let store_dir = path.parent().expect("file must have parent");
    fs::create_dir_all(store_dir)?;
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &serialize::FrecencyStoreSerializer::from(store))?;

    Ok(())
}

/// A collection of statistics about the stored items
pub struct FrecencyStore {
    reference_time: f64,
    half_life: f64,
    pub items: Vec<ItemStats>,
}

impl Default for FrecencyStore {
    fn default() -> FrecencyStore {
        FrecencyStore {
            reference_time: current_time_secs(),
            half_life: 60.0 * 60.0 * 24.0 * 3.0, // three day half life
            items: Vec::new(),
        }
    }
}

impl FrecencyStore {
    /// Remove all but the top N (sorted by `sort_method`) from the `UsageStore`
    pub fn truncate(&mut self, keep_num: usize, sort_method: SortMethod) {
        let mut sorted_vec = self.sorted(sort_method);
        sorted_vec.truncate(keep_num);
        self.items = sorted_vec;
    }

    /// Change the half life and reweight such that frecency does not change
    pub fn set_half_life(&mut self, half_life: f64) {
        self.reset_time();
        self.half_life = half_life;

        for item in self.items.iter_mut() {
            item.set_half_life(half_life);
        }
    }

    /// Return the number of half lives passed since the reference time
    pub fn half_lives_passed(&self) -> f64 {
        (current_time_secs() - self.reference_time) / self.half_life
    }

    /// Reset the reference time to now, and reweight all the statistics to reflect that
    pub fn reset_time(&mut self) {
        let current_time = current_time_secs();

        self.reference_time = current_time;

        for item in self.items.iter_mut() {
            item.reset_ref_time(current_time);
        }
    }

    /// Log a visit to a item
    pub fn add(&mut self, item: &str) {
        let item_stats = self.get(item);

        item_stats.update_frecency(1.0);
        item_stats.update_num_accesses(1);
        item_stats.update_last_access(current_time_secs());
    }

    /// Adjust the score of a item by a given weight
    pub fn adjust(&mut self, item: &str, weight: f64) {
        let item_stats = self.get(item);

        item_stats.update_frecency(weight);
        item_stats.update_num_accesses(weight as i32);
    }

    /// Delete an item from the store
    pub fn delete(&mut self, item: &str) {
        if let Some(idx) = self.items.iter().position(|i| i.item == item) {
            self.items.remove(idx);
        }
    }

    /// Return a sorted vector of all the items in the store, sorted by `sort_method`
    pub fn sorted(&self, sort_method: SortMethod) -> Vec<ItemStats> {
        let mut new_vec = self.items.clone();
        new_vec.sort_by(|item1, item2| item1.cmp_score(item2, sort_method).reverse());

        new_vec
    }

    /// Retrieve a mutable reference to a item in the store.
    /// If the item does not exist, create it and return a reference to the created item
    fn get(&mut self, item: &str) -> &mut ItemStats {
        match self
            .items
            .binary_search_by_key(&item, |item_stats| &item_stats.item)
        {
            Ok(idx) => &mut self.items[idx],
            Err(idx) => {
                self.items.insert(
                    idx,
                    ItemStats::new(item.to_string(), self.reference_time, self.half_life),
                );
                &mut self.items[idx]
            }
        }
    }
}

/// Print out all the items, sorted by `method`, with an optional maximum of `limit`
pub fn write_stats<W: Write>(
    w: &mut W,
    items: &[ItemStats],
    method: SortMethod,
    show_stats: bool,
    current_time: f64,
    precision_override: Option<usize>,
) -> Result<()> {
    for item in items {
        write_stat(
            w,
            item,
            method,
            show_stats,
            current_time,
            precision_override,
        )?;
    }
    Ok(())
}

pub fn write_stat<W: Write>(
    w: &mut W,
    item: &ItemStats,
    method: SortMethod,
    show_stats: bool,
    current_time: f64,
    precision_override: Option<usize>,
) -> Result<()> {
    if show_stats {
        let (score, default_precision) = match method {
            SortMethod::Recent => ((current_time - item.last_access()) / 60.0 / 60.0, 3),
            SortMethod::Frequent => (item.num_accesses as f64, 0),
            SortMethod::Frecent => (item.get_frecency(current_time), 3),
        };
        let precision = precision_override.unwrap_or(default_precision);
        w.write_fmt(format_args!(
            "{: <.prec$}\t{}\n",
            score,
            item.item,
            prec = precision
        ))?;
    } else {
        w.write_fmt(format_args!("{}\n", &item.item))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_usage() -> FrecencyStore {
        FrecencyStore {
            reference_time: current_time_secs(),
            half_life: 1.0,
            items: Vec::new(),
        }
    }

    #[test]
    fn add_new() {
        let mut usage = create_usage();

        usage.add("test");

        assert_eq!(1, usage.items.len());
    }

    #[test]
    fn add_existing() {
        let mut usage = create_usage();

        usage.add("test");
        usage.add("test");

        assert_eq!(1, usage.items.len());
    }

    #[test]
    fn delete_existing() {
        let mut usage = create_usage();
        usage.add("test");
        assert_eq!(usage.items.len(), 1);
        usage.delete("test");
        assert_eq!(usage.items.len(), 0);
    }

    #[test]
    fn delete_nonexisting() {
        let mut usage = create_usage();
        usage.delete("test");
        assert_eq!(usage.items.len(), 0);
    }

    #[test]
    fn adjust_existing() {
        let mut usage = create_usage();

        usage.add("test");
        usage.adjust("test", 3.0);

        assert_eq!(usage.items.len(), 1);
    }

    #[test]
    fn adjust_new() {
        let mut usage = create_usage();

        usage.adjust("test", 3.0);

        assert_eq!(usage.items.len(), 1);
    }

    #[test]
    fn truncate_greater() {
        let mut usage = create_usage();
        usage.add("dir1");
        usage.add("dir2");

        usage.truncate(1, SortMethod::Recent);

        assert_eq!(usage.items.len(), 1);
    }

    #[test]
    fn truncate_less() {
        let mut usage = create_usage();
        usage.add("dir1");
        usage.add("dir2");

        usage.truncate(3, SortMethod::Recent);

        assert_eq!(usage.items.len(), 2);
    }

    #[test]
    fn sorted_frecent() {
        let mut usage = create_usage();
        usage.add("dir1");
        usage.add("dir2");
        usage.get("dir2").update_frecency(1000.0);

        let sorted = usage.sorted(SortMethod::Frecent);

        assert_eq!(sorted.len(), 2);
        assert_eq!(sorted[0].item, "dir2".to_string());
    }

    #[test]
    fn sorted_frecent2() {
        let mut usage = create_usage();
        usage.add("dir1");
        usage.add("dir2");
        usage.get("dir1").update_frecency(1000.0);

        let sorted = usage.sorted(SortMethod::Frecent);

        assert_eq!(sorted.len(), 2);
        assert_eq!(sorted[0].item, "dir1".to_string());
    }

    #[test]
    fn sorted_recent() {
        let mut usage = create_usage();
        usage.add("dir1");
        usage.add("dir2");
        usage
            .get("dir2")
            .update_last_access(current_time_secs() + 100.0);

        let sorted = usage.sorted(SortMethod::Recent);

        assert_eq!(sorted.len(), 2);
        assert_eq!(sorted[0].item, "dir2".to_string());
    }

    #[test]
    fn sorted_frequent() {
        let mut usage = create_usage();
        usage.add("dir1");
        usage.add("dir2");
        usage.get("dir2").update_num_accesses(100);

        let sorted = usage.sorted(SortMethod::Frequent);

        assert_eq!(sorted.len(), 2);
        assert_eq!(sorted[0].item, "dir2".to_string());
    }

    #[test]
    fn get_exists() {
        let mut usage = create_usage();
        usage.add("dir1");

        let _stats = usage.get("dir1");

        assert_eq!(usage.items.len(), 1);
    }

    #[test]
    fn get_not_exists() {
        let mut usage = create_usage();
        usage.add("dir1");

        usage.get("dir2");

        assert_eq!(usage.items.len(), 2);
    }

    #[test]
    fn reset_time() {
        let mut usage = create_usage();
        let current_time = current_time_secs();
        usage.reference_time = current_time - 10.0;
        usage.add("test");
        let original_frecency = usage.get("test").get_frecency(current_time);

        usage.reset_time();

        assert!((usage.reference_time - current_time).abs() < 0.01);
        assert!((usage.get("test").get_frecency(current_time) - original_frecency).abs() < 0.01);
    }

    #[test]
    fn set_halflife() {
        let mut usage = create_usage();
        let current_time = current_time_secs();
        usage.reference_time = current_time - 10.0;
        usage.add("dir1");
        let original_frecency = usage.get("dir1").get_frecency(current_time);
        usage.set_half_life(10.0);

        let new_frecency = usage.get("dir1").get_frecency(current_time);

        assert!((usage.half_life - 10.0).abs() < 0.01);
        assert!((new_frecency - original_frecency).abs() < 0.01);
    }
}
