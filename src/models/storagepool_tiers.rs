

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolTiers {
  #[serde(rename = "tiers")]
  tiers: Option<Vec<::models::StoragepoolTierExtended>>
}

