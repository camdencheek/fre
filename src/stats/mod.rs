use super::current_time_secs;
use crate::args::SortMethod;
use std::cmp::Ordering;

pub mod serialize;

/// A representation of statistics for a single item
#[derive(Clone)]
pub struct ItemStats {
    pub item: String,
    half_life: f64,
    // Time in seconds since the epoch
    reference_time: f64,
    // Time in seconds since reference_time that this item was last accessed
    last_accessed: f64,
    frecency: f64,
    pub num_accesses: i32,
}

impl ItemStats {
    /// Create a new item
    pub fn new(item: String, ref_time: f64, half_life: f64) -> ItemStats {
        ItemStats {
            half_life,
            reference_time: ref_time,
            item,
            frecency: 0.0,
            last_accessed: 0.0,
            num_accesses: 0,
        }
    }

    /// Compare the score of two items given a sort method
    pub fn cmp_score(&self, other: &ItemStats, method: SortMethod) -> Ordering {
        match method {
            SortMethod::Frequent => self.cmp_frequent(other),
            SortMethod::Recent => self.cmp_recent(other),
            SortMethod::Frecent => self.cmp_frecent(other),
        }
    }

    /// Compare the frequency of two items
    fn cmp_frequent(&self, other: &ItemStats) -> Ordering {
        self.num_accesses.cmp(&other.num_accesses)
    }

    /// Compare the recency of two items
    fn cmp_recent(&self, other: &ItemStats) -> Ordering {
        self.last_accessed
            .partial_cmp(&other.last_accessed)
            .unwrap_or(Ordering::Less)
    }

    /// Compare the frecency of two items
    fn cmp_frecent(&self, other: &ItemStats) -> Ordering {
        self.frecency
            .partial_cmp(&other.frecency)
            .unwrap_or(Ordering::Less)
    }

    /// Change the half life of the item, maintaining the same frecency
    pub fn set_half_life(&mut self, half_life: f64) {
        let old_frecency = self.get_frecency(current_time_secs());
        self.half_life = half_life;
        self.set_frecency(old_frecency);
    }

    /// Calculate the frecency of the item
    pub fn get_frecency(&self, current_time_secs: f64) -> f64 {
        self.frecency / 2.0f64.powf((current_time_secs - self.reference_time) / self.half_life)
    }

    pub fn set_frecency(&mut self, new: f64) {
        self.frecency =
            new * 2.0f64.powf((current_time_secs() - self.reference_time) / self.half_life);
    }

    /// update the frecency of the item by the given weight
    pub fn update_frecency(&mut self, weight: f64) {
        let original_frecency = self.get_frecency(current_time_secs());
        self.set_frecency(original_frecency + weight);
    }

    /// Update the number of accesses of the item by the given weight
    pub fn update_num_accesses(&mut self, weight: i32) {
        self.num_accesses += weight;
    }

    /// Update the time the item was last accessed
    pub fn update_last_access(&mut self, time: f64) {
        self.last_accessed = time - self.reference_time;
    }

    /// Reset the reference time and recalculate the last_accessed time
    pub fn reset_ref_time(&mut self, new_time: f64) {
        let original_frecency = self.get_frecency(current_time_secs());
        let delta = self.reference_time - new_time;
        self.reference_time = new_time;
        self.last_accessed += delta;
        self.set_frecency(original_frecency);
    }

    /// Timestamp (in nanoseconds since epoch) of the last access
    pub fn last_access(&self) -> f64 {
        self.reference_time + self.last_accessed
    }
}

/// The number of seconds elapsed since `ref_time`
pub fn secs_elapsed(ref_time: f64) -> f64 {
    current_time_secs() - ref_time
}

#[cfg(test)]
mod tests {
    use crate::store::write_stat;

    use super::*;

    fn create_item() -> ItemStats {
        let test_item = "/test/item".to_string();

        ItemStats {
            half_life: 100.0,
            reference_time: current_time_secs(),
            item: test_item.clone(),
            frecency: 0.0,
            last_accessed: 0.0,
            num_accesses: 0,
        }
    }

    #[test]
    fn new_item_stats() {
        let test_item = "/test/item";
        let ref_time = current_time_secs();

        let new_item_stats = ItemStats::new(test_item.to_string(), ref_time, 10.0);

        assert_eq!(new_item_stats.frecency, 0.0);
        assert_eq!(new_item_stats.num_accesses, 0);
        assert_eq!(new_item_stats.frecency, 0.0);
    }

    #[test]
    fn compare_with_func() {
        let low_item_stats = create_item();
        let mut high_item_stats = create_item();

        high_item_stats.frecency = 1.0;
        high_item_stats.last_accessed = 1.0;
        high_item_stats.num_accesses = 1;

        assert_eq!(Ordering::Less, low_item_stats.cmp_frecent(&high_item_stats));
        assert_eq!(Ordering::Less, low_item_stats.cmp_recent(&high_item_stats));
        assert_eq!(
            Ordering::Less,
            low_item_stats.cmp_frequent(&high_item_stats)
        );
    }

    #[test]
    fn compare_with_enum() {
        let low_item_stats = create_item();
        let mut high_item_stats = create_item();

        high_item_stats.frecency = 1.0;
        high_item_stats.last_accessed = 1.0;
        high_item_stats.num_accesses = 1;

        assert_eq!(
            Ordering::Less,
            low_item_stats.cmp_score(&high_item_stats, SortMethod::Frecent)
        );
        assert_eq!(
            Ordering::Less,
            low_item_stats.cmp_score(&high_item_stats, SortMethod::Recent)
        );
        assert_eq!(
            Ordering::Less,
            low_item_stats.cmp_score(&high_item_stats, SortMethod::Frequent)
        );
    }

    #[test]
    fn update_score() {
        let mut stats = create_item();

        stats.update_frecency(1.0);

        assert!((stats.frecency - 1.0).abs() < 0.01);
        assert_eq!(stats.num_accesses, 0);
    }

    #[test]
    fn update_num_accesses() {
        let mut stats = create_item();

        stats.update_num_accesses(1);

        assert_eq!(stats.num_accesses, 1);
        assert!((stats.frecency.abs() - 0.0) < 0.01);
    }

    #[test]
    fn to_string_no_stats() {
        let stats = create_item();

        let t = current_time_secs();
        for method in [
            SortMethod::Frecent,
            SortMethod::Frequent,
            SortMethod::Recent,
        ] {
            let mut b = Vec::new();
            write_stat(&mut b, &stats, method, false, t, None).unwrap();
            assert_eq!(b, String::from("/test/item\n").into_bytes());
        }
    }

    #[test]
    fn to_string_stats() {
        let stats = create_item();

        let t = current_time_secs();
        for (method, expected) in [
            (SortMethod::Frecent, String::from("0.000\t/test/item\n")),
            (SortMethod::Recent, String::from("0.000\t/test/item\n")),
            (SortMethod::Frequent, String::from("0\t/test/item\n")),
        ] {
            let mut b = Vec::new();
            write_stat(&mut b, &stats, method, true, t, None).unwrap();
            assert_eq!(String::from_utf8(b).unwrap(), expected);
        }
    }

    #[test]
    fn to_string_custom_precision() {
        let t = current_time_secs();
        let mut stats = create_item();
        stats.reference_time = t - 100.654321;
        stats.last_accessed = 50.123456;
        stats.num_accesses = 15;
        stats.frecency = 320.123456;

        for (method, expected) in [
            (SortMethod::Frecent, String::from("159.33743\t/test/item\n")),
            (SortMethod::Recent, String::from("0.01404\t/test/item\n")),
            (SortMethod::Frequent, String::from("15.00000\t/test/item\n")),
        ] {
            let mut b = Vec::new();
            write_stat(&mut b, &stats, method, true, t, Some(5)).unwrap();
            assert_eq!(String::from_utf8(b).unwrap(), expected);
        }
    }

    #[test]
    fn get_frecency_one_half_life() {
        let mut stats = create_item();

        let t = current_time_secs();
        stats.reset_ref_time(t - 1.0 * stats.half_life);
        stats.frecency = 1.0;

        assert!((stats.get_frecency(t) - 0.5).abs() < 0.01);
    }

    #[test]
    fn get_frecency_two_half_lives() {
        let mut stats = create_item();

        let t = current_time_secs();
        stats.reset_ref_time(current_time_secs() - 2.0 * stats.half_life);
        stats.frecency = 1.0;

        assert!((stats.get_frecency(t) - 0.25).abs() < 0.01);
    }

    #[test]
    fn reset_time() {
        let mut low_item_stats = create_item();
        let current_time = current_time_secs();
        low_item_stats.reference_time = current_time - 5.0;
        low_item_stats.last_accessed = 10.0;

        low_item_stats.reset_ref_time(current_time);

        assert!((low_item_stats.reference_time - current_time).abs() < 0.1);
        assert!((low_item_stats.last_accessed - 5.0).abs() < 0.1);
    }

    #[test]
    fn set_half_life() {
        let mut low_item_stats = create_item();
        let current_time = current_time_secs();
        low_item_stats.reference_time = current_time - 2.0;
        low_item_stats.last_accessed = 1.0;
        let original_frecency = low_item_stats.get_frecency(current_time);

        low_item_stats.set_half_life(2.0);

        assert_eq!(low_item_stats.half_life, 2.0);
        assert!((low_item_stats.get_frecency(current_time) - original_frecency).abs() < 0.01);
    }
}
