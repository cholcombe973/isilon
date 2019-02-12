#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolStoragepools {
    #[serde(rename = "storagepools")]
    pub storagepools: Option<Vec <crate::models::StoragepoolStoragepool>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
