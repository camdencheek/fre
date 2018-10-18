use super::SortMethod;
use super::current_time_secs;
use std::cmp::Ordering;

pub mod serialize;

#[derive(Clone)]
pub struct PathStats {
    pub path: String,
    half_life: f32,
    reference_time: f64,
    frecency: f32,
    last_accessed: f32,
    num_accesses: i32
}

impl PathStats {
    pub fn new(path: String, ref_time: f64, half_life: f32) -> PathStats {
        PathStats {
            half_life: half_life,
            reference_time: ref_time,
            path: path.clone(),
            frecency: 0.0,
            last_accessed: 0.0,
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
        self.last_accessed
          .partial_cmp(&other.last_accessed)
          .unwrap_or(Ordering::Less)
    }

    fn cmp_frecent(&self, other: &PathStats) -> Ordering {
        self.frecency
            .partial_cmp(&other.frecency)
            .unwrap_or(Ordering::Less)
    }

    pub fn update_score(&mut self, weight: f32) {
        let elapsed_since_ref = secs_elapsed(self.reference_time);
        self.frecency += weight * 2.0f32.powf(elapsed_since_ref as f32 / self.half_life);
    }

    pub fn update_num_accesses(&mut self, weight: i32) {
        self.num_accesses += weight;
    }

    pub fn set_half_life(&mut self, half_life: f32) {
      self.frecency = self.frecency * 2.0f32.powf(
        secs_elapsed(self.reference_time) as f32 * (1.0 / self.half_life - 1.0 / half_life));

      self.half_life = half_life
    }

    pub fn get_frecency(&self) -> f32 {
        self.frecency / 2.0f32.powf(
          self.secs_since_access() as f32 / self.half_life)
    }

    pub fn update_last_access(&mut self, time: f64) {
        self.last_accessed = (time - self.reference_time) as f32;
    }

    pub fn reset_ref_time(&mut self, time: f64) {
      let delta = self.reference_time - time;
      self.reference_time = time;
      self.last_accessed += delta as f32;
    }

    pub fn secs_since_access(&self) -> f32 {
        secs_elapsed(self.reference_time) - self.last_accessed
    }

    pub fn to_string(&self, method: &SortMethod, show_stats: bool) -> String {
        if show_stats {
            match method {
                SortMethod::Recent => format!(
                    "{: <.3}\t{}\n",
                    self.secs_since_access() / 60.0 / 60.0,
                    self.path
                ),
                SortMethod::Frequent => format!(
                  "{: <}\t{}\n", 
                  self.num_accesses, 
                  self.path
                ),
                SortMethod::Frecent => format!(
                    "{: <.3}\t{}\n",
                    self.get_frecency(),
                    self.path
                ),
            }
        } else {
            return format!("{}\n", self.path.clone());
        }
    }
}

pub fn secs_elapsed(ref_time: f64) -> f32 {
  (current_time_secs() - ref_time) as f32
}


#[cfg(test)]
mod tests {
  use super::*;
  use spectral::prelude::*;

  fn create_path() -> PathStats {
    let test_path = "/test/path".to_string();
    let ref_time = current_time_secs();

    PathStats {
      half_life: 1.0,
      reference_time: ref_time,
      path: test_path.clone(),
      frecency: 0.0,
      last_accessed: 0.0,
      num_accesses: 0,
    }
  }

  #[test] 
  fn new_path_stats() {
    let test_path = "/test/path".to_string();
    let ref_time = current_time_secs();

    let new_path_stats = PathStats::new(test_path, ref_time, 10.0);

    assert_eq!(new_path_stats.frecency, 0.0);
    assert_eq!(new_path_stats.num_accesses, 0);
    assert_eq!(new_path_stats.frecency, 0.0);
  }


  #[test]
  fn compare_with_func() {
    let low_path_stats = create_path();
    let mut high_path_stats = create_path();

    high_path_stats.frecency = 1.0;
    high_path_stats.last_accessed = 1.0;
    high_path_stats.num_accesses = 1;

    assert_eq!(Ordering::Less, low_path_stats.cmp_frecent(&high_path_stats));
    assert_eq!(Ordering::Less, low_path_stats.cmp_recent(&high_path_stats));
    assert_eq!(Ordering::Less, low_path_stats.cmp_frequent(&high_path_stats));
  }


  #[test]
  fn compare_with_enum() {
    let low_path_stats = create_path();
    let mut high_path_stats = create_path();

    high_path_stats.frecency = 1.0;
    high_path_stats.last_accessed = 1.0;
    high_path_stats.num_accesses = 1;

    assert_eq!(Ordering::Less, low_path_stats.cmp_score(&high_path_stats, &SortMethod::Frecent));
    assert_eq!(Ordering::Less, low_path_stats.cmp_score(&high_path_stats, &SortMethod::Recent));
    assert_eq!(Ordering::Less, low_path_stats.cmp_score(&high_path_stats, &SortMethod::Frequent));
  }


  #[test]
  fn update_score() {
    let mut low_path_stats = create_path();

    low_path_stats.update_score(1.0);

    assert_that!(low_path_stats.frecency).is_close_to(1.0,0.01);
    assert_that!(low_path_stats.num_accesses).is_equal_to(0);
  }

  #[test]
  fn update_num_accesses() {
    let mut low_path_stats = create_path();

    low_path_stats.update_num_accesses(1);

    assert_that!(low_path_stats.num_accesses).is_equal_to(1);
    assert_that!(low_path_stats.frecency).is_close_to(0.0,0.01);
  }

  #[test]
  fn update_last_access() {
    let mut low_path_stats = create_path();

    low_path_stats.update_last_access(current_time_secs());

    assert_that!(low_path_stats.secs_since_access()).is_close_to(0.0, 0.1);
  }


  #[test]
  fn to_string_no_stats() {
    let low_path_stats = create_path();

    assert_that!(low_path_stats.to_string(&SortMethod::Frecent, false))
      .is_equal_to("/test/path\n".to_string());
    assert_that!(low_path_stats.to_string(&SortMethod::Recent, false))
      .is_equal_to("/test/path\n".to_string());
    assert_that!(low_path_stats.to_string(&SortMethod::Frequent, false))
      .is_equal_to("/test/path\n".to_string());
  }

  #[test]
  fn to_string_stats() {
    let low_path_stats = create_path();

    assert_that!(low_path_stats.to_string(&SortMethod::Frecent, true))
      .is_equal_to("0.000\t/test/path\n".to_string());
    assert_that!(low_path_stats.to_string(&SortMethod::Recent, true))
      .is_equal_to("0.000\t/test/path\n".to_string());
    assert_that!(low_path_stats.to_string(&SortMethod::Frequent, true))
      .is_equal_to("0\t/test/path\n".to_string());

  }

  #[test]
  fn get_frecency_one_half_life() {
    let mut low_path_stats = create_path();

    low_path_stats.frecency = 1.0;
    low_path_stats.last_accessed = -1.0;

    assert_that!(low_path_stats.get_frecency())
      .is_close_to(0.5, 0.1);
  }

  #[test]
  fn get_frecency_two_half_lives() {
    let mut low_path_stats = create_path();

    low_path_stats.frecency = 1.0;
    low_path_stats.last_accessed = -2.0;

    assert_that!(low_path_stats.get_frecency())
      .is_close_to(0.25, 0.1);
  }

  #[test]
  fn secs_since_access() {
    let mut low_path_stats = create_path();

    low_path_stats.last_accessed = -2.0;

    assert_that!(low_path_stats.secs_since_access()).is_close_to(2.0,0.1);
  }

  #[test]
  fn secs_elapsed_one_second() {
    let one_second_ago = current_time_secs() - 1.0;

    assert_that!(secs_elapsed(one_second_ago)).is_close_to(1.0, 0.1);
  }

  #[test]
  fn reset_time() {
    let mut low_path_stats = create_path();
    let current_time = current_time_secs();
    low_path_stats.reference_time = current_time - 5.0;
    low_path_stats.last_accessed = 10.0;

    low_path_stats.reset_ref_time(current_time);

    assert_that!(low_path_stats.reference_time).is_close_to(current_time, 0.1);
    assert_that!(low_path_stats.last_accessed).is_close_to(5.0, 0.1);
  }

  #[test]
  fn set_half_life() {
    let mut low_path_stats = create_path();
    let current_time = current_time_secs();
    low_path_stats.reference_time = current_time - 2.0;
    low_path_stats.last_accessed = 1.0;
    let original_frecency = low_path_stats.get_frecency();

    low_path_stats.set_half_life(2.0);

    assert_that!(low_path_stats.half_life).is_equal_to(2.0);
    assert_that!(low_path_stats.get_frecency()).is_close_to(original_frecency, 0.01);
  }
}
