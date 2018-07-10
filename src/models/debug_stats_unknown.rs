

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugStatsUnknown {
  /// The number of calls.
  #[serde(rename = "calls")]
  calls: Option<i32>,
  /// The number of errors.
  #[serde(rename = "errors")]
  errors: Option<i32>,
  /// The total amount of time spent in this method.
  #[serde(rename = "time")]
  time: Option<f32>
}

