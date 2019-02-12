/// ClusterNode : Node information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNode {
    /// List of the drives in this node.
    #[serde(rename = "drives")]
    pub drives: Option<Vec <crate::models::ClusterNodeDrive>>,
    /// Node state information (reported and modifiable).
    #[serde(rename = "state")]
    pub state: Option <crate::models::ClusterNodeState>,
}
