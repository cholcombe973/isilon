#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodesExtendedExtended {
    #[serde(rename = "nodes")]
    pub nodes: Option<Vec<::models::ClusterNodeExtended>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
