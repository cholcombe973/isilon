

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolStatus {
  /// Disk pools which are currently unhealthy.
  #[serde(rename = "unhealthy")]
  unhealthy: Vec<::models::StoragepoolStatusUnhealthyItem>,
  /// Drives which are not currently provisioned into a disk pool.
  #[serde(rename = "unprovisioned")]
  unprovisioned: Vec<::models::StoragepoolStatusUnprovisionedItem>
}

