

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsGlobalExtended {
  /// Settings for Audit.
  #[serde(rename = "settings")]
  settings: Option<::models::SettingsGlobalSettings>
}

