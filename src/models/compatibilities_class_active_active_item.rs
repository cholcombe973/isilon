

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilitiesClassActiveActiveItem {
  /// The first class in an active compatibility
  #[serde(rename = "class_1")]
  class_1: String,
  /// The second class in an active compatibility
  #[serde(rename = "class_2")]
  class_2: String,
  /// The id of this active compatibility
  #[serde(rename = "id")]
  id: i32
}

