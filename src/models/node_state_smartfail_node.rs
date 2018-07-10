#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStateSmartfailNode {
    /// This node is dead (dead_devs).
    #[serde(rename = "dead")]
    pub dead: Option<bool>,
    /// This node is down (down_devs).
    #[serde(rename = "down")]
    pub down: Option<bool>,
    /// Error message, if the HTTP status returned from this node was not 200.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Node ID of the node reporting this information.
    #[serde(rename = "id")]
    pub id: Option<i32>,
    /// This node is in the cluster (all_devs).
    #[serde(rename = "in_cluster")]
    pub in_cluster: Option<bool>,
    /// Logical node number of the node reporting this information.
    #[serde(rename = "lnn")]
    pub lnn: Option<i32>,
    /// This node is readonly (ro_devs).
    #[serde(rename = "readonly")]
    pub readonly: Option<bool>,
    /// This node is shutdown readonly (down_devs).
    #[serde(rename = "shutdown_readonly")]
    pub shutdown_readonly: Option<bool>,
    /// This node is smartfailed (soft_devs).
    #[serde(rename = "smartfailed")]
    pub smartfailed: Option<bool>,
    /// Status of the HTTP response from this node if not 200.  If 200, this field does not appear.
    #[serde(rename = "status")]
    pub status: Option<i32>,
}
