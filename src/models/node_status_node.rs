#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatusNode {
    /// Battery status information.
    #[serde(rename = "batterystatus")]
    pub batterystatus: Option <crate::models::NodeStatusNodeBatterystatus>,
    /// Storage capacity of this node.
    #[serde(rename = "capacity")]
    pub capacity: Option<Vec <crate::models::NodeStatusNodeCapacityItem>>,
    /// CPU status information for this node.
    #[serde(rename = "cpu")]
    pub cpu: Option <crate::models::NodeStatusNodeCpu>,
    /// Error message, if the HTTP status returned from this node was not 200.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Node ID of the node reporting this information.
    #[serde(rename = "id")]
    pub id: Option<i32>,
    /// Logical node number of the node reporting this information.
    #[serde(rename = "lnn")]
    pub lnn: Option<i32>,
    /// Node NVRAM information.
    #[serde(rename = "nvram")]
    pub nvram: Option <crate::models::NodeStatusNodeNvram>,
    /// Information about this node's power supplies.
    #[serde(rename = "powersupplies")]
    pub powersupplies: Option <crate::models::NodeStatusNodePowersupplies>,
    /// OneFS release.
    #[serde(rename = "release")]
    pub release: Option<String>,
    /// Status of the HTTP response from this node if not 200.  If 200, this field does not appear.
    #[serde(rename = "status")]
    pub status: Option<i32>,
    /// Seconds this node has been online.
    #[serde(rename = "uptime")]
    pub uptime: Option<i32>,
    /// OneFS version.
    #[serde(rename = "version")]
    pub version: Option<String>,
}
