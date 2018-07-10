#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudSettingsSettingsSleepTimeoutCloudGarbageCollection {
    /// Sleep timeout for a recovery thread with pending tasks
    #[serde(rename = "recovery_with_tasks")]
    pub recovery_with_tasks: Option<f32>,
    /// Sleep timeout for a recovery thread without pending tasks
    #[serde(rename = "recovery_without_tasks")]
    pub recovery_without_tasks: Option<f32>,
    /// Sleep timeout for a non-recovery thread with pending tasks
    #[serde(rename = "with_tasks")]
    pub with_tasks: Option<f32>,
    /// Sleep timeout for a non-recovery thread without pending tasks
    #[serde(rename = "without_tasks")]
    pub without_tasks: Option<f32>,
}
