

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryFile {
  #[serde(rename = "statistics")]
  statistics: Option<Vec<::models::HistoryFileStatistic>>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

