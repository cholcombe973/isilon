

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventCategory {
  /// Numeric identifier of eventgroup category.
  #[serde(rename = "id")]
  id: Option<String>,
  /// Name of category.
  #[serde(rename = "id_name")]
  id_name: Option<String>,
  /// Description of category.
  #[serde(rename = "name")]
  name: Option<String>
}

