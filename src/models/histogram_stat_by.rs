

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistogramStatBy {
  /// Unix Epoch time of start of results collection job.
  #[serde(rename = "begin_time")]
  begin_time: i32,
  /// Histogram breakout data according to breakout parameter.
  #[serde(rename = "breakouts")]
  breakouts: Vec<::models::HistogramStatByBreakout>,
  /// Total length of the result list.
  #[serde(rename = "result_length")]
  result_length: i32
}

