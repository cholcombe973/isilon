

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpSettings {
  /// This is schema that contains HTTP protocol properties.
  #[serde(rename = "settings")]
  settings: Option<::models::HttpSettingsSettings>
}

