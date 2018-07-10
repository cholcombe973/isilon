

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WormSettings {
  /// Specifies global SmartLock (WORM) settings.
  #[serde(rename = "settings")]
  settings: Option<::models::WormSettingsSettings>
}

