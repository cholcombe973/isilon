

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileFilterSettings {
  /// 
  #[serde(rename = "settings")]
  settings: ::models::FileFilterSettingsSettings
}

