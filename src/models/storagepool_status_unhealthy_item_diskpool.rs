#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolStatusUnhealthyItemDiskpool {
    /// The drives that are part of this disk pool.
    #[serde(rename = "drives")]
    pub drives: Vec <crate::models::StoragepoolStatusUnprovisionedItem>,
    /// The system ID given to the disk pool.
    #[serde(rename = "id")]
    pub id: i32,
    /// The disk pool name.
    #[serde(rename = "name")]
    pub name: String,
    /// The system ID of the disk pool's node pool, if it is in a node pool.
    #[serde(rename = "nodepool_id")]
    pub nodepool_id: Option<i32>,
    /// The protection policy for the disk pool.
    #[serde(rename = "protection_policy")]
    pub protection_policy: String,
    /// The SSDs that are part of this disk pool.
    #[serde(rename = "ssd_drives")]
    pub ssd_drives: Vec <crate::models::StoragepoolStatusUnprovisionedItem>,
}
