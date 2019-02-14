#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilitiesClassAvailable {
    #[serde(rename = "available")]
    pub available: Option<Vec <crate::models::CompatibilitiesClassAvailableAvailableItem>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
