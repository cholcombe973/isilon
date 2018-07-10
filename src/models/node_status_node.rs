

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatusNode {
  /// Battery status information.
  #[serde(rename = "batterystatus")]
  batterystatus: Option<::models::NodeStatusNodeBatterystatus>,
  /// Storage capacity of this node.
  #[serde(rename = "capacity")]
  capacity: Option<Vec<::models::NodeStatusNodeCapacityItem>>,
  /// CPU status information for this node.
  #[serde(rename = "cpu")]
  cpu: Option<::models::NodeStatusNodeCpu>,
  /// Error message, if the HTTP status returned from this node was not 200.
  #[serde(rename = "error")]
  error: Option<String>,
  /// Node ID of the node reporting this information.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// Logical node number of the node reporting this information.
  #[serde(rename = "lnn")]
  lnn: Option<i32>,
  /// Node NVRAM information.
  #[serde(rename = "nvram")]
  nvram: Option<::models::NodeStatusNodeNvram>,
  /// Information about this node's power supplies.
  #[serde(rename = "powersupplies")]
  powersupplies: Option<::models::NodeStatusNodePowersupplies>,
  /// OneFS release.
  #[serde(rename = "release")]
  release: Option<String>,
  /// Status of the HTTP response from this node if not 200.  If 200, this field does not appear.
  #[serde(rename = "status")]
  status: Option<i32>,
  /// Seconds this node has been online.
  #[serde(rename = "uptime")]
  uptime: Option<i32>,
  /// OneFS version.
  #[serde(rename = "version")]
  version: Option<String>
}

