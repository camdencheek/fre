use super::super::stats::serialize;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct FrecencyStoreSerializer {
    reference_time: f64,
    half_life: f32,
    items: Vec<serialize::ItemStatsSerializer>,
}

impl From<FrecencyStore> for FrecencyStoreSerializer {
    fn from(store: FrecencyStore) -> Self {
        let items = store
            .items
            .into_iter()
            .map(serialize::ItemStatsSerializer::from)
            .collect();

        FrecencyStoreSerializer {
            reference_time: store.reference_time,
            half_life: store.half_life,
            items,
        }
    }
}

impl From<FrecencyStoreSerializer> for FrecencyStore {
    fn from(store: FrecencyStoreSerializer) -> Self {
        let ref_time = store.reference_time;
        let half_life = store.half_life;
        let items = store
            .items
            .into_iter()
            .map(|s| s.into_item_stats(ref_time, half_life))
            .collect();

        FrecencyStore {
            reference_time: store.reference_time,
            half_life: store.half_life,
            items,
        }
    }
}
