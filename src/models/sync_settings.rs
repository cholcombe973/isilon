

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncSettings {
  /// Global SyncIQ settings.
  #[serde(rename = "settings")]
  settings: Option<::models::SyncSettingsSettings>
}

