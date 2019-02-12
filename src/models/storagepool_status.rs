#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolStatus {
    /// Disk pools which are currently unhealthy.
    #[serde(rename = "unhealthy")]
    pub unhealthy: Vec <crate::models::StoragepoolStatusUnhealthyItem>,
    /// Drives which are not currently provisioned into a disk pool.
    #[serde(rename = "unprovisioned")]
    pub unprovisioned: Vec <crate::models::StoragepoolStatusUnprovisionedItem>,
}
