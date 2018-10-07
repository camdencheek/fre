use super::{
  store::UsageStore,
  stats::PathStats
};
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::{self, BufReader, BufWriter};


#[derive(Serialize,Deserialize,Debug)]
struct PathStatsSerializer {
    path: String,
    frecency: f32,
    last_accessed: f32,
    num_accesses: i32,
}

#[derive(Serialize,Deserialize,Debug)]
struct UsageStoreSerializer {
    reference_time: f64,
    half_life: f32,
    paths: Vec<PathStatsSerializer>,
}

impl From<UsageStore> for UsageStoreSerializer {
  fn from(store: UsageStore) -> Self {
    let paths = store.paths
      .into_iter()
      .map(PathStatsSerializer::from)
      .collect();

    UsageStoreSerializer {
      reference_time: store.reference_time,
      half_life: store.half_life,
      paths: paths
    }
  }
}

impl From<PathStats> for PathStatsSerializer {
  fn from(stats: PathStats) -> Self {
    PathStatsSerializer {
      path: stats.path,
      frecency: stats.frecency,
      last_accessed: stats.last_accessed,
      num_accesses: stats.num_accesses
    }
  }
}

impl From<UsageStoreSerializer> for UsageStore {
  fn from(store: UsageStoreSerializer) -> Self {
    let ref_time = store.reference_time;
    let half_life = store.half_life;
    let paths = store.paths
      .into_iter()
      .map(|serializer| {
        PathStats {
          half_life: half_life,
          reference_time: ref_time,
          path: serializer.path,
          frecency: serializer.frecency,
          last_accessed: serializer.last_accessed,
          num_accesses: serializer.num_accesses
        }
      })
    .collect();

    UsageStore {
      reference_time: store.reference_time,
      half_life: store.half_life,
      paths: paths
    }
  }
} 

pub fn read_store(path: &PathBuf) -> Result<UsageStore, io::Error> {
    if path.is_file() {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let store: UsageStoreSerializer = serde_json::from_reader(reader)?;
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
    serde_json::to_writer_pretty(writer, &UsageStoreSerializer::from(store))?;

    return Ok(());
}
