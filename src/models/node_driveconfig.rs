#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDriveconfig {
    #[serde(rename = "nodes")]
    pub nodes: Option<Vec<crate::models::NodeDriveconfigNode>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
