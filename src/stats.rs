use super::SortMethod;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::cmp::Ordering;
use std::rc::Rc;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PathStats {
    half_life: Rc<f64>,
    reference_time: Rc<NaiveDateTime>,
    pub path: String,
    pub frecency: f64,
    pub last_accessed: i64,
    pub num_accesses: i64,
}

impl PathStats {
    pub fn new(path: String, ref_time: NaiveDateTime, half_life: f64) -> PathStats {
        PathStats {
            half_life: Rc::new(half_life),
            reference_time: Rc::new(ref_time),
            path: path.clone(),
            frecency: 0.0,
            last_accessed: 0,
            num_accesses: 0,
        }
    }

    pub fn cmp_score(&self, other: &PathStats, method: &SortMethod) -> Ordering {
        match method {
            SortMethod::Frequent => self.cmp_frequent(other),
            SortMethod::Recent => self.cmp_recent(other),
            SortMethod::Frecent => self.cmp_frecent(other),
        }
    }

    fn cmp_frequent(&self, other: &PathStats) -> Ordering {
        self.num_accesses.cmp(&other.num_accesses)
    }

    fn cmp_recent(&self, other: &PathStats) -> Ordering {
        self.last_accessed.cmp(&other.last_accessed)
    }

    fn cmp_frecent(&self, other: &PathStats) -> Ordering {
        self.frecency
            .partial_cmp(&other.frecency)
            .unwrap_or(Ordering::Less)
    }

    pub fn update_score(&mut self, weight: f64) {
        let elapsed_since_ref = secs_elapsed(*self.reference_time);
        self.frecency += weight * 2.0f64.powf(elapsed_since_ref as f64 / *self.half_life);
    }

    pub fn update_num_accesses(&mut self, weight: i64) {
        self.num_accesses += weight;
    }

    pub fn update_last_access(&mut self) {
        self.last_accessed = secs_elapsed(*self.reference_time);
    }

    pub fn secs_since_access(&self) -> i64 {
        secs_elapsed(*self.reference_time) - self.last_accessed
    }

    pub fn to_string(&self, method: &SortMethod, show_stats: bool) -> String {
        if show_stats {
            match method {
                SortMethod::Recent => format!(
                    "{: <.3}\t{}\n",
                    self.secs_since_access() as f64 / 60.0 / 60.0,
                    self.path
                ),
                SortMethod::Frequent => format!("{: <}\t{}\n", self.num_accesses, self.path),
                SortMethod::Frecent => format!(
                    "{: <.3}\t{}\n",
                    self.frecency / 2.0f64.powf(self.secs_since_access() as f64 / *self.half_life),
                    self.path
                ),
            }
        } else {
            return format!("{}\n", self.path.clone());
        }
    }
}

pub fn secs_elapsed(ref_time: NaiveDateTime) -> i64 {
    DateTime::<Utc>::from(SystemTime::now())
        .naive_local()
        .signed_duration_since(ref_time)
        .num_seconds()
}


#[cfg(test)]
mod tests {
  use super::*;
  use spectral::prelude::*;

  fn create_low_path() -> PathStats {
    let test_path = "/test/path".to_string();
    let ref_time = DateTime::<Utc>::from(SystemTime::now()).naive_local();

    PathStats {
      half_life: Rc::new(10.0),
      reference_time: Rc::new(ref_time),
      path: test_path.clone(),
      frecency: 1.0,
      last_accessed: 100,
      num_accesses: 1,
    }
  }

  fn create_high_path() -> PathStats {
    let test_path = "/test/path".to_string();
    let ref_time = DateTime::<Utc>::from(SystemTime::now()).naive_local();

    PathStats {
      half_life: Rc::new(10.0),
      reference_time: Rc::new(ref_time),
      path: test_path.clone(),
      frecency: 2.0,
      last_accessed: 200,
      num_accesses: 2,
    }
  }

  #[test] 
  fn new_path_stats() {
    let test_path = "/test/path".to_string();
    let ref_time = DateTime::<Utc>::from(SystemTime::now()).naive_local();

    let new_path_stats = PathStats::new(test_path, ref_time, 10.0);

    assert_eq!(new_path_stats.frecency, 0.0);
    assert_eq!(new_path_stats.num_accesses, 0);
    assert_eq!(new_path_stats.frecency, 0.0);
  }


  #[test]
  fn compare_with_func() {
    let low_path_stats = create_low_path();
    let high_path_stats = create_high_path();

    assert_eq!(Ordering::Less, low_path_stats.cmp_frecent(&high_path_stats));
    assert_eq!(Ordering::Less, low_path_stats.cmp_recent(&high_path_stats));
    assert_eq!(Ordering::Less, low_path_stats.cmp_frequent(&high_path_stats));
  }


  #[test]
  fn compare_with_enum() {
    let low_path_stats = create_low_path();
    let high_path_stats = create_high_path();

    assert_eq!(Ordering::Less, low_path_stats.cmp_score(&high_path_stats, &SortMethod::Frecent));
    assert_eq!(Ordering::Less, low_path_stats.cmp_score(&high_path_stats, &SortMethod::Recent));
    assert_eq!(Ordering::Less, low_path_stats.cmp_score(&high_path_stats, &SortMethod::Frequent));
  }


  #[test]
  fn update_score() {
    let mut low_path_stats = create_low_path();

    low_path_stats.update_score(1.0);

    assert_that!(low_path_stats.frecency).is_close_to(2.0,0.01);
    assert_that!(low_path_stats.num_accesses).is_equal_to(1);
  }

  #[test]
  fn update_num_accesses() {
    let mut low_path_stats = create_low_path();

    low_path_stats.update_num_accesses(1);

    assert_that!(low_path_stats.num_accesses).is_equal_to(2);
    assert_that!(low_path_stats.frecency).is_close_to(1.0,0.01);
  }

  #[test]
  fn update_last_access() {
    let mut low_path_stats = create_low_path();

    low_path_stats.update_last_access();

    assert_that!(low_path_stats.secs_since_access()).is_equal_to(0);
  }


  #[test]
  fn to_string() {
    let low_path_stats = create_low_path();

    assert_that!(low_path_stats.to_string(&SortMethod::Frecent, false))
      .is_equal_to("/test/path\n".to_string())
  }
}
