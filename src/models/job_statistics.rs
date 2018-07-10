

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatistics {
  #[serde(rename = "jobs")]
  jobs: Option<Vec<::models::JobStatisticsJob>>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

