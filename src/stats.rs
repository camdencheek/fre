use std::time::{SystemTime, UNIX_EPOCH};
use std::cmp::Ordering;
use super::SortMethod;
use chrono::{DateTime,NaiveDateTime,Utc};

const HALF_LIFE: f64 = 60.0 * 60.0 * 24.0 * 7.0; // one week in seconds


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DirectoryStats {
    pub directory: String,
    pub frecency: f64,
    pub last_accessed: i64, 
    pub num_accesses: u64,
}

impl DirectoryStats {
    pub fn new(path: String, ref_time: NaiveDateTime) -> DirectoryStats {
      let mut ds = DirectoryStats {
        directory: path.clone(),
        frecency: 0.0,
        last_accessed: 0,
        num_accesses: 0,
      };

      ds.increase(1.0, ref_time);

      ds
    }

    fn cmp_frequent(&self, other: &DirectoryStats) -> Ordering {
        self.num_accesses.cmp(&other.num_accesses)
    }

    fn cmp_recent(&self, other: &DirectoryStats) -> Ordering {
        self.last_accessed.cmp(&other.last_accessed)
    }

    fn cmp_frecent(&self, other: &DirectoryStats) -> Ordering {
        self.frecency.partial_cmp(&other.frecency).unwrap_or(Ordering::Less)
    }

    pub fn cmp_score(&self, other: &DirectoryStats, method: &SortMethod) -> Ordering {
        match method {
            SortMethod::Frequent => self.cmp_frequent(other),
            SortMethod::Recent => self.cmp_recent(other),
            SortMethod::Frecent => self.cmp_frecent(other),
        }
    }

    pub fn increase(&mut self, weight: f64, ref_time: NaiveDateTime) {
      self.num_accesses += weight as u64;
      let elapsed_since_ref = DateTime::<Utc>::from(SystemTime::now())
        .naive_local()
        .signed_duration_since(ref_time)
        .num_seconds();
      self.last_accessed = elapsed_since_ref;
      self.frecency += weight *
        2.0f64.powf(elapsed_since_ref as f64 / HALF_LIFE);
    }

    pub fn decrease(&mut self, weight: f64, ref_time: NaiveDateTime) {
      self.num_accesses += weight as u64;
      let elapsed_since_ref = DateTime::<Utc>::from(SystemTime::now())
        .naive_local()
        .signed_duration_since(ref_time)
        .num_seconds();
      self.frecency -= weight *
        2.0f64.powf(elapsed_since_ref as f64 / HALF_LIFE);
      // TODO determine how last accessed should be handled
    }
    
    pub fn secs_since_access(&self, ref_time: NaiveDateTime) -> i64 {
      secs_elapsed(ref_time) - self.last_accessed
    }

    pub fn to_string(&self, method: &SortMethod, show_stats: bool, ref_time: NaiveDateTime) -> String {
      if show_stats {
        match method {
          SortMethod::Recent => format!("{: <.3}\t{}\n", self.secs_since_access(ref_time) as f64 / 60.0 / 60.0, self.directory),
          SortMethod::Frequent => format!("{: <}\t{}\n", self.num_accesses, self.directory),
          SortMethod::Frecent => format!("{: <.3}\t{}\n", self.frecency / 2.0f64.powf(self.secs_since_access(ref_time) as f64 / HALF_LIFE), self.directory),
        }
      } else {
        match method {
          _ => format!("{}\n",self.directory.clone()),
        }
      }
    }
}

pub fn secs_elapsed(ref_time: NaiveDateTime) -> i64 {
  DateTime::<Utc>::from(SystemTime::now()).naive_local()
    .signed_duration_since(ref_time)
    .num_seconds()
}
