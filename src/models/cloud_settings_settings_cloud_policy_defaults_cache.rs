

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudSettingsSettingsCloudPolicyDefaultsCache {
  /// Specifies cache expiration.
  #[serde(rename = "expiration")]
  expiration: Option<i32>,
  /// Specifies cache read ahead type.
  #[serde(rename = "read_ahead")]
  read_ahead: Option<String>,
  /// Specifies cache type.
  #[serde(rename = "type")]
  _type: Option<String>
}

