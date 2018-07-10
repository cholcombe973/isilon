/// ClusterNodesExtended : View information about nodes during an upgrade, rollback, or pre-upgrade assessment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodesExtended {
    /// List of detailed info of nodes which are part of the current upgrade
    #[serde(rename = "nodes")]
    pub nodes: Option<Vec<::models::ClusterNodes>>,
    /// Total number of nodes the upgrade framework is aware of and was able to collect info for at this point.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
