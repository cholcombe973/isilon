#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodesAvailableNode {
    /// Node configuration ID.
    #[serde(rename = "configuration_id")]
    pub configuration_id: Option<String>,
    /// Human-readable description giving further detail on status (may be empty)
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Isilon product name.
    #[serde(rename = "product")]
    pub product: Option<String>,
    /// Serial number of this node.
    #[serde(rename = "serial_number")]
    pub serial_number: Option<String>,
    /// Availability of the node.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// OneFS build version running on the node.
    #[serde(rename = "version")]
    pub version: Option<String>,
}
