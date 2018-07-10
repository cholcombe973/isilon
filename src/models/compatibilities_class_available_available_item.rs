

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilitiesClassAvailableAvailableItem {
  /// The first class in an available compatibility
  #[serde(rename = "class_1")]
  class_1: String,
  /// The second class in an available compatibility
  #[serde(rename = "class_2")]
  class_2: String
}

