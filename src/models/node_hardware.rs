#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeHardware {
    #[serde(rename = "nodes")]
    pub nodes: Option<Vec <crate::models::NodeHardwareNode>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
