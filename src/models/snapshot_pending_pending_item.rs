

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotPendingPendingItem {
  /// The system supplied unique ID used for sorting and paging.
  #[serde(rename = "id")]
  id: String,
  /// The /ifs path that will snapshotted.
  #[serde(rename = "path")]
  path: String,
  /// The name of the schedule used to create this snapshot.
  #[serde(rename = "schedule")]
  schedule: String,
  /// The system snapshot name formed from the schedule formate.
  #[serde(rename = "snapshot")]
  snapshot: String,
  /// The Unix Epoch time the snapshot will be created.
  #[serde(rename = "time")]
  time: i32
}

