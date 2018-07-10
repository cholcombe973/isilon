#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsReportsExtended {
    /// The directory on /ifs where manual or live reports will be placed.
    #[serde(rename = "live_dir")]
    pub live_dir: Option<String>,
    /// The number of manual reports to keep.
    #[serde(rename = "live_retain")]
    pub live_retain: Option<i32>,
    /// The isidate schedule used to generate reports.
    #[serde(rename = "schedule")]
    pub schedule: Option<String>,
    /// The directory on /ifs where schedule reports will be placed.
    #[serde(rename = "scheduled_dir")]
    pub scheduled_dir: Option<String>,
    /// The number of scheduled reports to keep.
    #[serde(rename = "scheduled_retain")]
    pub scheduled_retain: Option<i32>,
}
