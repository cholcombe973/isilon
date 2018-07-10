

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsHistoryStatValue {
  /// Unix Epoch time in seconds that statistic was collected.
  #[serde(rename = "time")]
  time: i32,
  /// Key dependent value.
  #[serde(rename = "value")]
  value: String
}

