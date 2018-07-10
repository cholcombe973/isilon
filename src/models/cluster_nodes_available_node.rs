

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodesAvailableNode {
  /// Node configuration ID.
  #[serde(rename = "configuration_id")]
  configuration_id: Option<String>,
  /// Human-readable description giving further detail on status (may be empty)
  #[serde(rename = "description")]
  description: Option<String>,
  /// Isilon product name.
  #[serde(rename = "product")]
  product: Option<String>,
  /// Serial number of this node.
  #[serde(rename = "serial_number")]
  serial_number: Option<String>,
  /// Availability of the node.
  #[serde(rename = "status")]
  status: Option<String>,
  /// OneFS build version running on the node.
  #[serde(rename = "version")]
  version: Option<String>
}

