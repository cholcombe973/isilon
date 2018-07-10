

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsKeyPolicy {
  /// Time between samples in seconds.
  #[serde(rename = "interval")]
  interval: i32,
  /// If true, history is persisted.
  #[serde(rename = "persistent")]
  persistent: bool,
  /// Time in seconds to keep data.
  #[serde(rename = "retention")]
  retention: i32
}

