

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSettingsSettingsMaintenance {
  /// Duration of maintenance period in seconds.
  #[serde(rename = "duration")]
  duration: Option<i32>,
  /// Start of maintenance period.
  #[serde(rename = "start")]
  start: Option<i32>
}

