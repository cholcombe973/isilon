#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeHardwareFast {
    #[serde(rename = "nodes")]
    pub nodes: Option<Vec<::models::NodeHardwareFastNode>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
