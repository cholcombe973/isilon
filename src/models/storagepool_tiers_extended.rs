#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolTiersExtended {
    #[serde(rename = "tiers")]
    pub tiers: Option<Vec<::models::StoragepoolTierExtended>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
