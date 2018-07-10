

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsVariable {
  /// The value of environment variable.
  #[serde(rename = "value")]
  value: String
}

