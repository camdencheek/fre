use std::time::{SystemTime, UNIX_EPOCH};
use std::cmp::Ordering;
use super::SortMethod;

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryStats {
    pub score: f64,
    pub last_accessed: u64,
    pub num_accesses: u64,
}

impl DirectoryStats {
    pub fn cmp_frequent(&self, other: &DirectoryStats) -> Ordering {
        self.num_accesses.cmp(&other.num_accesses)
    }

    pub fn cmp_recent(&self, other: &DirectoryStats) -> Ordering {
        self.last_accessed.cmp(&other.last_accessed)
    }

    pub fn cmp_frecent(&self, other: &DirectoryStats) -> Ordering {
        self.score.partial_cmp(&other.score).unwrap_or(Ordering::Less)
    }

    pub fn cmp(&self, other: &DirectoryStats, method: &SortMethod) -> Ordering {
        match method {
            SortMethod::Frequent => self.cmp_frequent(other),
            SortMethod::Recent => self.cmp_recent(other),
            SortMethod::Frecent => self.cmp_frecent(other),
        }
    }
}

impl Default for DirectoryStats {
    fn default() -> DirectoryStats {
        DirectoryStats {
            last_accessed: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            num_accesses: 0,
            score: 0.0,
        }
    }
}
