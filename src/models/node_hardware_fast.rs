

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeHardwareFast {
  #[serde(rename = "nodes")]
  nodes: Option<Vec<::models::NodeHardwareFastNode>>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

