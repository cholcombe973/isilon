

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistogramStatByBreakout {
  /// List of bucket, file count pairs.
  #[serde(rename = "data")]
  data: Vec<Vec<i32>>,
  /// Breakout key by which results are filtered.
  #[serde(rename = "key")]
  key: String
}

