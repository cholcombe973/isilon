#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilitiesSsdAvailable {
    #[serde(rename = "available")]
    pub available: Option<Vec <crate::models::CompatibilitiesSsdAvailableAvailableItem>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
