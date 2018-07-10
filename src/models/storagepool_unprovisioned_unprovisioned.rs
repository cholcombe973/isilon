

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolUnprovisionedUnprovisioned {
  /// A list of unprovisioned drives that do not belong to an unprovisioned node.
  #[serde(rename = "drives")]
  drives: Vec<::models::StoragepoolStatusUnprovisionedItem>,
  /// A list of lnns whose drives are all unprovisioned
  #[serde(rename = "lnns")]
  lnns: Vec<i32>
}

