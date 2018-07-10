

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsCharacterEncodings {
  /// 
  #[serde(rename = "settings")]
  settings: Option<::models::SettingsCharacterEncodingsSettings>
}

