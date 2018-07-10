

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudSettingsSettingsSleepTimeoutCloudGarbageCollection {
  /// Sleep timeout for a recovery thread with pending tasks
  #[serde(rename = "recovery_with_tasks")]
  recovery_with_tasks: Option<f32>,
  /// Sleep timeout for a recovery thread without pending tasks
  #[serde(rename = "recovery_without_tasks")]
  recovery_without_tasks: Option<f32>,
  /// Sleep timeout for a non-recovery thread with pending tasks
  #[serde(rename = "with_tasks")]
  with_tasks: Option<f32>,
  /// Sleep timeout for a non-recovery thread without pending tasks
  #[serde(rename = "without_tasks")]
  without_tasks: Option<f32>
}

