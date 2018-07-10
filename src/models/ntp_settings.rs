

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NtpSettings {
  /// NTP settings.
  #[serde(rename = "settings")]
  settings: Option<::models::NtpSettingsSettings>
}

