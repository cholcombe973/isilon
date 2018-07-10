

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotSchedules {
  #[serde(rename = "schedules")]
  schedules: Option<Vec<::models::SnapshotScheduleExtended>>
}

