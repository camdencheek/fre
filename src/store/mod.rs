mod serialize;

use super::stats::PathStats;
use super::current_time_secs;
use super::SortMethod;
use std::default::Default;
use std::io::{self, BufWriter,BufReader, Write};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::process;
use log::error;

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

pub fn write_store(store: UsageStore, path: &PathBuf) -> io::Result<()> {
    let store_dir = path.parent().expect("file must have parent");
    fs::create_dir_all(&store_dir)?;
    let file = File::create(&path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &serialize::UsageStoreSerializer::from(store))?;

    return Ok(());
}

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
    pub fn purge(&mut self) {
        self.paths.retain(|dir| Path::new(&dir.path).exists());
    }


    pub fn truncate(&mut self, keep_num: usize, sort_method: &SortMethod) {
        let mut sorted_vec = self.sorted(sort_method);
        sorted_vec.truncate(keep_num);
        self.paths = sorted_vec;
    }

    pub fn reset_time(&mut self) {
      let current_time = current_time_secs();
      let delta = current_time - self.reference_time;
      
      self.reference_time = current_time;

      for path in self.paths.iter_mut() {
        path.reference_time = current_time;
        path.last_accessed -= delta as f32;
      }
    }

    pub fn add(&mut self, path: &str) {
        let path_stats = self.get(&path);

        path_stats.update_score(1.0);
        path_stats.update_num_accesses(1);
        path_stats.update_last_access();
    }

    pub fn adjust(&mut self, path: &str, weight: f32) {
        let path_stats = self.get(&path);

        path_stats.update_score(weight);
        path_stats.update_num_accesses(weight as i32);
    }


    pub fn print_sorted(&self, method: &SortMethod, show_stats: bool, limit: Option<usize>) {

        let stdout = io::stdout();
        let handle = stdout.lock();
        let mut writer = BufWriter::new(handle);

        let sorted = self.sorted(method);
        let take_num = limit.unwrap_or(sorted.len());

        for dir in sorted.iter().take(take_num) {
          writer.write_all(dir.to_string(method, show_stats).as_bytes())
            .unwrap_or_else(|e| {
              error!("unable to write to stdout: {}", e);
              process::exit(1);
            });
        }
    }

    fn sorted(&self, sort_method: &SortMethod) -> Vec<PathStats> {
        let mut new_vec = self.paths.clone();
        new_vec.sort_by(|dir1, dir2| {
          dir1.cmp_score(dir2, sort_method).reverse()
        });

        new_vec
    }

    fn get(&mut self, path: &str) -> &mut PathStats {
        match self.paths.binary_search_by_key(&path, |dir_stats| &dir_stats.path) {
            Ok(idx) => return &mut self.paths[idx],
            Err(idx) => {
                self.paths.insert(
                    idx,
                    PathStats::new(path.to_string(), self.reference_time, self.half_life),
                );
                return &mut self.paths[idx]
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
      paths: Vec::new()
    }
  }

  #[test]
  fn add_new() {
    let mut usage = create_usage();

    usage.add("test");

    assert_that!(usage.paths.len()).is_equal_to(1);
    assert_that!(usage.get("test").half_life).is_close_to(1.0, 0.001);
    assert_that!(usage.get("test").num_accesses).is_equal_to(1);
    assert_that!(usage.get("test").frecency).is_close_to(1.0, 0.01);
    assert_that!(usage.get("test").last_accessed).is_close_to(0.0, 0.1);
  }

  #[test]
  fn add_existing() {
    let mut usage = create_usage();

    usage.add("test");
    usage.add("test");

    assert_that!(usage.paths.len()).is_equal_to(1);
    assert_that!(usage.get("test").half_life).is_close_to(1.0, 0.001);
    assert_that!(usage.get("test").num_accesses).is_equal_to(2);
    assert_that!(usage.get("test").frecency).is_close_to(2.0, 0.01);
    assert_that!(usage.get("test").last_accessed).is_close_to(0.0, 0.1);
  }

  #[test]
  fn adjust_existing() {
    let mut usage = create_usage();

    usage.add("test");
    usage.adjust("test", 3.0);

    assert_that!(usage.paths.len()).is_equal_to(1);
    assert_that!(usage.get("test").half_life).is_close_to(1.0, 0.001);
    assert_that!(usage.get("test").num_accesses).is_equal_to(4);
    assert_that!(usage.get("test").frecency).is_close_to(4.0, 0.01);
    assert_that!(usage.get("test").last_accessed).is_close_to(0.0, 0.1);
  }

  #[test]
  fn adjust_new() {
    let mut usage = create_usage();
     
    usage.adjust("test", 3.0);

    assert_that!(usage.paths.len()).is_equal_to(1);
    assert_that!(usage.get("test").half_life).is_close_to(1.0, 0.001);
    assert_that!(usage.get("test").num_accesses).is_equal_to(3);
    assert_that!(usage.get("test").frecency).is_close_to(3.0, 0.01);
    assert_that!(usage.get("test").last_accessed).is_close_to(0.0, 0.1);
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
    usage.get("dir2").frecency = 100.0;

    let sorted = usage.sorted(&SortMethod::Frecent);

    assert_that!(sorted.len()).is_equal_to(2);
    assert_that!(sorted[0].path).is_equal_to("dir2".to_string());
  }

  #[test]
  fn sorted_recent() {
    let mut usage = create_usage();
    usage.add("dir1");
    usage.add("dir2");
    usage.get("dir2").last_accessed = 100.0;

    let sorted = usage.sorted(&SortMethod::Recent);

    assert_that!(sorted.len()).is_equal_to(2);
    assert_that!(sorted[0].path).is_equal_to("dir2".to_string());
  }

  #[test]
  fn sorted_frequent() {
    let mut usage = create_usage();
    usage.add("dir1");
    usage.add("dir2");
    usage.get("dir2").num_accesses = 100;

    let sorted = usage.sorted(&SortMethod::Frequent);

    assert_that!(sorted.len()).is_equal_to(2);
    assert_that!(sorted[0].path).is_equal_to("dir2".to_string());
  }

  #[test]
  fn get_exists() {
    let mut usage = create_usage();
    usage.adjust("dir1", 100.0);

    {
      let stats = usage.get("dir1");
      assert_that!(stats.num_accesses).is_equal_to(100);
    }

    assert_that!(usage.paths.len()).is_equal_to(1);
  }

  #[test]
  fn get_not_exists() {
    let mut usage = create_usage();
    usage.add("dir1");

    {
      let stats = usage.get("dir2");
      assert_that!(stats.path).is_equal_to("dir2".to_string());
    }

    assert_that!(usage.paths.len()).is_equal_to(2);
  }

  #[test]
  fn reset_time() {
    let mut usage = create_usage();
    let current_time = current_time_secs();
    usage.reference_time = current_time - 10.0;
    usage.get("test1").last_accessed = 5.0;

    usage.reset_time(); 

    assert_that!(usage.reference_time).is_close_to(current_time, 0.1);
    assert_that!(usage.get("test1").reference_time).is_close_to(current_time,0.1);
    assert_that!(usage.get("test1").last_accessed).is_close_to(-5.0,0.1);
  }
}
