#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolStatusUnhealthyItemAffectedItem {
    ///
    #[serde(rename = "device")]
    pub device: Option<::models::StoragepoolStatusUnprovisionedItem>,
    /// Whether or not the device is currently down.
    #[serde(rename = "down")]
    pub down: bool,
    /// Whether or not the device is currently being repaired.
    #[serde(rename = "restriping")]
    pub restriping: bool,
    /// Whether or not the device is currently smartfailed.
    #[serde(rename = "smartfailed")]
    pub smartfailed: bool,
    /// The type of affected device.
    #[serde(rename = "type")]
    pub _type: String,
}
