

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryFileStatistic {
  /// Nodes allocated for the sync action.
  #[serde(rename = "allocated")]
  allocated: i32,
  /// An ID for a single performance report.
  #[serde(rename = "id")]
  id: i32,
  /// Sync action limit.
  #[serde(rename = "limit")]
  limit: i32,
  /// Timestamp for the performance report.
  #[serde(rename = "timestamp")]
  timestamp: i32,
  /// Total usage for the performance report.
  #[serde(rename = "total")]
  total: i32
}

