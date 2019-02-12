/// ClusterFirmwareStatus : The firmware status for the cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterFirmwareStatus {
    /// List of the firmware status information for all the nodes in the cluster.
    #[serde(rename = "nodes")]
    pub nodes: Option<Vec <crate::models::ClusterFirmwareStatusNode>>,
}
