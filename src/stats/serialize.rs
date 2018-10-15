use super::*;

#[derive(Serialize,Deserialize,Debug)]
pub struct PathStatsSerializer {
    pub path: String,
    pub frecency: f32,
    pub last_accessed: f32,
    pub num_accesses: i32,
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

impl PathStatsSerializer {
  pub fn into_path_stats(self, ref_time: f64, half_life: f32) -> PathStats {
    PathStats {
      half_life: half_life,
      reference_time: ref_time,
      path: self.path,
      frecency: self.frecency,
      last_accessed: self.last_accessed,
      num_accesses: self.num_accesses
    }
  }
}

