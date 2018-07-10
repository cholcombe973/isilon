#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolStatusUnprovisionedItem {
    /// The drive bay number.
    #[serde(rename = "bay")]
    pub bay: i32,
    /// The node the drive is on.
    #[serde(rename = "lnn")]
    pub lnn: i32,
}
