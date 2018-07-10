

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolSettingsSettingsSpilloverTarget {
  /// Target pool ID if target specified, otherwise null.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// Target pool name if target specified, otherwise null.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Type of target pool.
  #[serde(rename = "type")]
  _type: String
}

