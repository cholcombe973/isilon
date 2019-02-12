#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilitiesClassActiveExtended {
    #[serde(rename = "active")]
    pub active: Option<Vec <crate::models::CompatibilitiesClassActiveActiveItem>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
