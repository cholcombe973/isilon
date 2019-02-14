/// ClusterNodes : The node details useful during an upgrade or assessment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodes {
    /// The current OneFS version before upgrade.
    #[serde(rename = "error")]
    pub error: Option <crate::models::ClusterNodesError>,
    /// The last action performed to completion/failure on this node.  Null if the node_state is 'committed' or 'assessing.' One of the following values: 'upgrade', 'rollback'.
    #[serde(rename = "last_action")]
    pub last_action: Option<String>,
    /// Did the node pass upgrade or rollback without failing? Null if the node_state is 'committed.' One of the following values: 'pass', 'fail', null
    #[serde(rename = "last_action_result")]
    pub last_action_result: Option<String>,
    /// The lnn of the node.
    #[serde(rename = "lnn")]
    pub lnn: Option<i32>,
    /// \\e The state of the node during the upgrade, rollback, or assessment. One of the following values: 'committed', 'upgraded', 'upgrading', 'rolling back', 'assessing', 'error'
    #[serde(rename = "node_state")]
    pub node_state: Option<String>,
    /// The current OneFS version before upgrade.
    #[serde(rename = "onefs_version")]
    pub onefs_version: Option <crate::models::ClusterNodesOnefsVersion>,
    /// What step is the upgrade, assessment, or rollback in? To show via progress indicator. NOTE: the value is an integer between 0 and 100 (percent)
    #[serde(rename = "progress")]
    pub progress: Option<i32>,
}
