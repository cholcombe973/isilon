#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSettingsSettingsMaintenance {
    /// Duration of maintenance period in seconds.
    #[serde(rename = "duration")]
    pub duration: Option<i32>,
    /// Start of maintenance period.
    #[serde(rename = "start")]
    pub start: Option<i32>,
}
