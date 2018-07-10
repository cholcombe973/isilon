

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotSettings {
  /// 
  #[serde(rename = "settings")]
  settings: Option<::models::SnapshotSettingsSettings>
}

