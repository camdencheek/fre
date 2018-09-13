use rayon::slice::ParallelSliceMut;
use std::collections::HashMap;
use serde_json;
use super::{default_store_path, SortMethod};
use super::stats::DirectoryStats;
use std::time::SystemTime;
use std::io::{self, BufReader, BufWriter, Write, Stdout};
use std::fs::{self, File};
use std::path::{PathBuf, Path};
use std::default::Default;

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
  reference_time: SystemTime,
  half_life_secs: u64,
  directories: Vec<DirectoryStats>,
}

impl Default for Store {
  fn default() ->  Store {
    Store {
      reference_time: SystemTime::now(),
      half_life_secs: 60 * 60 * 24 * 7 * 2, // two week half life
      directories: Vec::new(),
    }
  }
}

impl Store {
  pub fn purge(&mut self) {
    self.directories.retain(|dir| Path::new(&dir.directory).exists());
  }

  pub fn sorted(&self, sort_method: &SortMethod) -> Vec<DirectoryStats> {
    let mut new_vec = self.directories.clone();
    
    new_vec.par_sort_by(|dir1, dir2| {
      dir1.cmp_score(dir2, sort_method).reverse()
    });

    new_vec
  }

  pub fn truncate(&mut self, keep_num: usize, sort_method: &SortMethod) {
    unimplemented!();
  }

  pub fn reset_time(time: SystemTime) {
    unimplemented!();
  }

  pub fn add(&mut self, path: String) {
    let ref_time = self.reference_time.clone();
    if let Some(dir) = self.find_mut(&path) {
      dir.increase(1.0, ref_time);
    } else {
        let index = self.directories
          .binary_search_by_key(&path, |dir_stats| dir_stats.directory.clone())
          .err()
          .unwrap();
        
        self.directories.insert(index, DirectoryStats::new(path.clone(), ref_time));
    }
  }

  pub fn find(&self, path: &String) -> Option<&DirectoryStats> {
    match self.directories.binary_search_by_key(&path, |dirStats| &dirStats.directory) {
      Ok(index) => Some(&self.directories[index]),
      Err(index) => None,
    }
  }

  pub fn find_mut(&mut self, path: &String) -> Option<&mut DirectoryStats> {
    match self.directories.binary_search_by_key(&path, |dirStats| &dirStats.directory) {
      Ok(index) => Some(&mut self.directories[index]),
      Err(index) => None,
    }
  }

  pub fn print_sorted(&self, method: &SortMethod, show_stats: bool, limit: Option<u64>) {
    let sorted = self.sorted(method);

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut writer = BufWriter::new(handle);

    match limit {
      Some(n) => {
        for dir in sorted.iter().take(n as usize) {
            writer.write(dir.to_string(method, show_stats, self.reference_time).as_bytes());
        }
      },
      None => {
        for dir in sorted.iter() {
            writer.write(dir.to_string(method, show_stats, self.reference_time).as_bytes());
        }
      }
    }
  }
}

// TODO return a result
pub fn read_store(path: &PathBuf) -> Store {
  let usage: Store = if path.is_file() {
    let file = File::open(&path)
      .expect(&format!("Cannot open file {}", &path.to_str().unwrap()));
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Cannot unmarshal json from storage file")
  } else {
    Store {
      reference_time: SystemTime::now(),
      half_life_secs: 60 * 60 * 24 * 7 * 2, // two week half life
      directories: Vec::new(),
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
