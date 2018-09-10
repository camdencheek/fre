use rayon::slice::ParallelSliceMut;
use std::collections::HashMap;
use serde_json;
use super::{default_store_path, SortMethod};
use super::stats::DirectoryStats;
use std::time::SystemTime;
use std::io::{self, BufReader, BufWriter, Write};
use std::fs::{self, File};
use std::path::{PathBuf, Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
  time_created: SystemTime,
  half_life_secs: u64,
  pub directories: HashMap<String, DirectoryStats>,
}

impl Store {
  pub fn purge(&mut self) {
    self.directories.retain(|dir, _| Path::new(&dir).exists());
  }

  pub fn sorted(&self, sort_method: &SortMethod) -> Vec<(String,String)> {
    let mut unsorted_vector: Vec<_> = self.directories
      .iter()
      .collect();

    unsorted_vector
      .par_sort_by(|(_, val1), (_, val2)| val1.cmp(val2, sort_method).reverse());

    unsorted_vector
      .iter()
      .map(|(dir,stats)| (dir,stats.score_string(sort_method)))
      .collect()

  }

  pub fn truncate(&mut self, keep_num: usize, sort_method: &SortMethod) {
    let sorted = self.sorted(sort_method);
    for (dir, _) in sorted.iter().skip(keep_num) {
      self.directories.remove(dir);
    }
  }

  pub fn reset_time(time: SystemTime) {
    unimplemented!();
  }

  pub fn access_dir(&mut self, path: String) {
    self.promote_dir(path, 1.0);
  }

  pub fn promote_dir(&mut self, path: String, amount: f64) {
    let stats = self.directories
      .entry(path)
      .or_insert(DirectoryStats::default());

    stats.num_accesses += amount as u64;
    stats.last_accessed = self.time_created
      .elapsed()
      .expect("Time went backward")
      .as_secs();
    stats.score += amount *
      2.0f64.powf(stats.last_accessed as f64 / self.half_life_secs as f64);
  }

  pub fn demote_dir(&mut self, path: String, amount: f64) {
    let stats = self.directories
      .entry(path)
      .or_insert(DirectoryStats::default());

    stats.num_accesses += amount as u64;
    stats.score -= amount *
      2.0f64.powf(stats.last_accessed as f64 / self.half_life_secs as f64);
  }

  pub fn print_sorted(&self, limit: Option<u64>, method: &SortMethod, stat: bool) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut writer = BufWriter::new(handle);

    if stat {
      for  in self.sorted(method).iter() {
        writer.write(&format!("{} {}",dir).into_bytes());
      }
    } else {
      for dir in self.sorted(method).iter() {
        writer.write(&format!("{}",dir).into_bytes());
      }
    }
  }
}

pub fn read_store(path: &PathBuf) -> Store {
  let usage: Store = if path.is_file() {
    let file = File::open(&path)
      .expect(&format!("Cannot open file {}", &path.to_str().unwrap()));
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Cannot unmarshal json from storage file")
  } else {
    Store {
      time_created: SystemTime::now(),
      half_life_secs: 60 * 60 * 24 * 7 * 2, // two week half life
      directories: HashMap::new(),
    }
  };

  return usage;
}

pub fn write_store(d: &Store, path: &PathBuf) {
  let store_dir = path.parent().expect("file must have parent");
  fs::create_dir_all(&store_dir).unwrap();
  let file = File::create(&path).unwrap();
  let writer = BufWriter::new(file);
  serde_json::to_writer(writer, &d).unwrap();
}
