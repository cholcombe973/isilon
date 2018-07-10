

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolNodepoolsExtended {
  #[serde(rename = "nodepools")]
  nodepools: Option<Vec<::models::StoragepoolNodepoolExtended>>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

