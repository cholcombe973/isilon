

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolStatusUnhealthyItemDiskpool {
  /// The drives that are part of this disk pool.
  #[serde(rename = "drives")]
  drives: Vec<::models::StoragepoolStatusUnprovisionedItem>,
  /// The system ID given to the disk pool.
  #[serde(rename = "id")]
  id: i32,
  /// The disk pool name.
  #[serde(rename = "name")]
  name: String,
  /// The system ID of the disk pool's node pool, if it is in a node pool.
  #[serde(rename = "nodepool_id")]
  nodepool_id: Option<i32>,
  /// The protection policy for the disk pool.
  #[serde(rename = "protection_policy")]
  protection_policy: String,
  /// The SSDs that are part of this disk pool.
  #[serde(rename = "ssd_drives")]
  ssd_drives: Vec<::models::StoragepoolStatusUnprovisionedItem>
}

