

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolStatusUnhealthyItemAffectedItem {
  /// 
  #[serde(rename = "device")]
  device: Option<::models::StoragepoolStatusUnprovisionedItem>,
  /// Whether or not the device is currently down.
  #[serde(rename = "down")]
  down: bool,
  /// Whether or not the device is currently being repaired.
  #[serde(rename = "restriping")]
  restriping: bool,
  /// Whether or not the device is currently smartfailed.
  #[serde(rename = "smartfailed")]
  smartfailed: bool,
  /// The type of affected device.
  #[serde(rename = "type")]
  _type: String
}

