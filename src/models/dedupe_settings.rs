

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DedupeSettings {
  /// Dedupe settings.
  #[serde(rename = "settings")]
  settings: Option<::models::DedupeSettingsSettings>
}

