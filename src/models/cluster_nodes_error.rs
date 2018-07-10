#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodesError {
    /// Last upgrade step which failed on node.
    #[serde(rename = "failed_upgrade_action")]
    pub failed_upgrade_action: Option<String>,
    /// Upgrade error log.
    #[serde(rename = "log")]
    pub log: Option<String>,
}
