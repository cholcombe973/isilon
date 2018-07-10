#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotSchedulesExtended {
    /// Resume token value to use in subsequent calls for continuation.
    #[serde(rename = "resume")]
    pub resume: Option<String>,
    #[serde(rename = "schedules")]
    pub schedules: Option<Vec<::models::SnapshotScheduleExtendedExtended>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
