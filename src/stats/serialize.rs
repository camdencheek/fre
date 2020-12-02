use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemStatsSerializer {
    pub item: String,
    pub frecency: f32,
    pub last_accessed: f32,
    pub num_accesses: i32,
}

impl From<ItemStats> for ItemStatsSerializer {
    fn from(stats: ItemStats) -> Self {
        ItemStatsSerializer {
            item: stats.item,
            frecency: stats.frecency,
            last_accessed: stats.last_accessed,
            num_accesses: stats.num_accesses,
        }
    }
}

impl ItemStatsSerializer {
    pub fn into_item_stats(self, ref_time: f64, half_life: f32) -> ItemStats {
        ItemStats {
            half_life,
            reference_time: ref_time,
            item: self.item,
            frecency: self.frecency,
            last_accessed: self.last_accessed,
            num_accesses: self.num_accesses,
        }
    }
}
