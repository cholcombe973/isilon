

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeHardware {
  #[serde(rename = "nodes")]
  nodes: Option<Vec<::models::NodeHardwareNode>>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

