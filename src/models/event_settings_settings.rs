#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSettingsSettings {
    /// Interval between heartbeat events. \"daily\", \"weekly\", or \"monthly\".
    #[serde(rename = "heartbeat_interval")]
    pub heartbeat_interval: Option<String>,
    /// Specifies start and duration of maintenance period during which no alerts will be sent for new eventgroups.
    #[serde(rename = "maintenance")]
    pub maintenance: Option <crate::models::EventSettingsSettingsMaintenance>,
    /// Retention period in days
    #[serde(rename = "retention_days")]
    pub retention_days: Option<i32>,
    /// Storage limit in megabytes per terabyte of available storage
    #[serde(rename = "storage_limit")]
    pub storage_limit: Option<i32>,
}
