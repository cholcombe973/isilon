#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsKeyPolicy {
    /// Time between samples in seconds.
    #[serde(rename = "interval")]
    pub interval: i32,
    /// If true, history is persisted.
    #[serde(rename = "persistent")]
    pub persistent: bool,
    /// Time in seconds to keep data.
    #[serde(rename = "retention")]
    pub retention: i32,
}
