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
