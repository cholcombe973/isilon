#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WormSettingsSettings {
    /// Specifies the current time of the SmartLock compliance clock in Unix Epoch seconds. If the compliance clock is not set, this value is null. A PUT request will set the compliance clock date to the current system time. The cluster must be in compliance mode to set the compliance clock.
    #[serde(rename = "cdate")]
    pub cdate: Option<i32>,
}
