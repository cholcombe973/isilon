#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterConfigDevice {
    /// Device ID.
    #[serde(rename = "devid")]
    pub devid: i32,
    /// Device GUID.
    #[serde(rename = "guid")]
    pub guid: String,
    /// If true, this node is online and communicating with the local node and every other node with the is_up property normally.  If false, this node is not currently communicating with the local node or other nodes with the is_up property.  It may be shut down, rebooting, disconnected from the backend network, or connected only to other nodes.
    #[serde(rename = "is_up")]
    pub is_up: bool,
    /// Device logical node number.
    #[serde(rename = "lnn")]
    pub lnn: i32,
}
