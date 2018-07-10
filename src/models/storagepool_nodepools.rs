#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolNodepools {
    #[serde(rename = "nodepools")]
    pub nodepools: Option<Vec<::models::StoragepoolNodepoolExtended>>,
}
