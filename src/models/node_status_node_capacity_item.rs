

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatusNodeCapacityItem {
  /// Total device storage bytes.
  #[serde(rename = "bytes")]
  bytes: Option<i32>,
  /// Total device count.
  #[serde(rename = "count")]
  count: Option<i32>,
  /// Device type.
  #[serde(rename = "type")]
  _type: Option<String>
}

