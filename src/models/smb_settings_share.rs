

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbSettingsShare {
  /// 
  #[serde(rename = "settings")]
  settings: ::models::SmbSettingsShareSettings
}

