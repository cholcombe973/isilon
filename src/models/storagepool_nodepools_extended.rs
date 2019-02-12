#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolNodepoolsExtended {
    #[serde(rename = "nodepools")]
    pub nodepools: Option<Vec <crate::models::StoragepoolNodepoolExtended>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
