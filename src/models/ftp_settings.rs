

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FtpSettings {
  /// FTP settings.
  #[serde(rename = "settings")]
  settings: Option<::models::FtpSettingsSettings>
}

