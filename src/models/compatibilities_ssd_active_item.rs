

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilitiesSsdActiveItem {
  /// Do not create ssd compatibility, only assess if creation is possible.
  #[serde(rename = "assess")]
  assess: Option<bool>,
  /// The node class of the desired ssd compatibility
  #[serde(rename = "class_1")]
  class_1: String,
  /// The optional second node class to turn on ssd compatibility
  #[serde(rename = "class_2")]
  class_2: Option<String>,
  /// Is this SSD Compatibility Count Compatible.
  #[serde(rename = "count")]
  count: Option<bool>
}

