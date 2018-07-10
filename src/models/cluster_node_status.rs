

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodeStatus {
  /// Battery status information.
  #[serde(rename = "batterystatus")]
  batterystatus: Option<::models::NodeStatusNodeBatterystatus>,
  /// Storage capacity of this node.
  #[serde(rename = "capacity")]
  capacity: Option<Vec<::models::NodeStatusNodeCapacityItem>>,
  /// CPU status information for this node.
  #[serde(rename = "cpu")]
  cpu: Option<::models::NodeStatusNodeCpu>,
  /// Node NVRAM information.
  #[serde(rename = "nvram")]
  nvram: Option<::models::NodeStatusNodeNvram>,
  /// Information about this node's power supplies.
  #[serde(rename = "powersupplies")]
  powersupplies: Option<::models::NodeStatusNodePowersupplies>,
  /// OneFS release.
  #[serde(rename = "release")]
  release: Option<String>,
  /// Seconds this node has been online.
  #[serde(rename = "uptime")]
  uptime: Option<i32>,
  /// OneFS version.
  #[serde(rename = "version")]
  version: Option<String>
}

