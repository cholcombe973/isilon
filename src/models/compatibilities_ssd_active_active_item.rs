

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilitiesSsdActiveActiveItem {
  /// The node class of an ssd compatibility
  #[serde(rename = "class_1")]
  class_1: String,
  /// Is this SSD Compatibility Count Compatible.
  #[serde(rename = "count")]
  count: bool,
  /// The id of this ssd compatibility
  #[serde(rename = "id")]
  id: i32
}

