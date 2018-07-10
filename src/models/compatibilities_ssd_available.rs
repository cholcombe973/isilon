

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilitiesSsdAvailable {
  #[serde(rename = "available")]
  available: Option<Vec<::models::CompatibilitiesSsdAvailableAvailableItem>>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

