

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsCharacterEncodingsSettings {
  /// Current character encoding.
  #[serde(rename = "current-encoding")]
  current_encoding: String,
  /// Default character encoding.
  #[serde(rename = "default-encoding")]
  default_encoding: String,
  /// A list of supported encoding values.
  #[serde(rename = "encodings")]
  encodings: Vec<String>
}

