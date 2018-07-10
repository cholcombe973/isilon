

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSnapshotScheduleResponse {
  /// The ID of the newly created snapshot schedule.
  #[serde(rename = "id")]
  id: i32
}

