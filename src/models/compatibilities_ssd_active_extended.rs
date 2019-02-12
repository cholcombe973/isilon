#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilitiesSsdActiveExtended {
    #[serde(rename = "active")]
    pub active: Option<Vec<crate::models::CompatibilitiesSsdActiveActiveItem>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
