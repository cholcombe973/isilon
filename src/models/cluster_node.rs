
/// ClusterNode : Node information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNode {
  /// List of the drives in this node.
  #[serde(rename = "drives")]
  drives: Option<Vec<::models::ClusterNodeDrive>>,
  /// Node state information (reported and modifiable).
  #[serde(rename = "state")]
  state: Option<::models::ClusterNodeState>
}

