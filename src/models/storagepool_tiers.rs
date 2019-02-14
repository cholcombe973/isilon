#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolTiers {
    #[serde(rename = "tiers")]
    pub tiers: Option<Vec <crate::models::StoragepoolTierExtended>>,
}
