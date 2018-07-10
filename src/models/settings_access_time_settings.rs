#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsAccessTimeSettings {
    /// Enable access time tracking.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Access time tracked in seconds for each cluster file if enabled.
    #[serde(rename = "precision")]
    pub precision: i32,
}
