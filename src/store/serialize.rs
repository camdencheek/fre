use super::super::stats::serialize;
use super::*;

#[derive(Serialize,Deserialize,Debug)]
pub struct UsageStoreSerializer {
    reference_time: f64,
    half_life: f32,
    paths: Vec<serialize::PathStatsSerializer>,
}

impl From<UsageStore> for UsageStoreSerializer {
  fn from(store: UsageStore) -> Self {
    let paths = store.paths
      .into_iter()
      .map(serialize::PathStatsSerializer::from)
      .collect();

    UsageStoreSerializer {
      reference_time: store.reference_time,
      half_life: store.half_life,
      paths: paths
    }
  }
}

impl From<UsageStoreSerializer> for UsageStore {
  fn from(store: UsageStoreSerializer) -> Self {
    let ref_time = store.reference_time;
    let half_life = store.half_life;
    let paths = store.paths
      .into_iter()
      .map(|s| s.into_path_stats(ref_time, half_life))
      .collect();

    UsageStore {
      reference_time: store.reference_time,
      half_life: store.half_life,
      paths: paths
    }
  }
} 
