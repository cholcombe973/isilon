

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotSchedulesExtended {
  /// Resume token value to use in subsequent calls for continuation.
  #[serde(rename = "resume")]
  resume: Option<String>,
  #[serde(rename = "schedules")]
  schedules: Option<Vec<::models::SnapshotScheduleExtendedExtended>>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

