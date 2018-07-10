

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolTiersExtended {
  #[serde(rename = "tiers")]
  tiers: Option<Vec<::models::StoragepoolTierExtended>>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

