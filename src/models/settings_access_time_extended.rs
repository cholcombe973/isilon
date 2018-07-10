#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsAccessTimeExtended {
    /// Enable access time tracking.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Access time tracked on each cluster file accurate to this number of seconds.
    #[serde(rename = "precision")]
    pub precision: Option<i32>,
}
