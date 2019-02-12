#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotSchedules {
    #[serde(rename = "schedules")]
    pub schedules: Option<Vec <crate::models::SnapshotScheduleExtended>>,
}
