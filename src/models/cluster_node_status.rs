#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodeStatus {
    /// Battery status information.
    #[serde(rename = "batterystatus")]
    pub batterystatus: Option<::models::NodeStatusNodeBatterystatus>,
    /// Storage capacity of this node.
    #[serde(rename = "capacity")]
    pub capacity: Option<Vec<::models::NodeStatusNodeCapacityItem>>,
    /// CPU status information for this node.
    #[serde(rename = "cpu")]
    pub cpu: Option<::models::NodeStatusNodeCpu>,
    /// Node NVRAM information.
    #[serde(rename = "nvram")]
    pub nvram: Option<::models::NodeStatusNodeNvram>,
    /// Information about this node's power supplies.
    #[serde(rename = "powersupplies")]
    pub powersupplies: Option<::models::NodeStatusNodePowersupplies>,
    /// OneFS release.
    #[serde(rename = "release")]
    pub release: Option<String>,
    /// Seconds this node has been online.
    #[serde(rename = "uptime")]
    pub uptime: Option<i32>,
    /// OneFS version.
    #[serde(rename = "version")]
    pub version: Option<String>,
}
