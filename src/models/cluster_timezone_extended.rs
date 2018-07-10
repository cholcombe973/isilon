
/// ClusterTimezoneExtended : The cluster timezone settings.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterTimezoneExtended {
  /// 
  #[serde(rename = "settings")]
  settings: Option<::models::ClusterTimezoneSettingsExtended>
}

