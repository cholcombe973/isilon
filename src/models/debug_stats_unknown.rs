#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugStatsUnknown {
    /// The number of calls.
    #[serde(rename = "calls")]
    pub calls: Option<i32>,
    /// The number of errors.
    #[serde(rename = "errors")]
    pub errors: Option<i32>,
    /// The total amount of time spent in this method.
    #[serde(rename = "time")]
    pub time: Option<f32>,
}
