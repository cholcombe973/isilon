
/// SyncReportsRotate : Force rotation of reports.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncReportsRotate {
  /// A message about the status of the task.
  #[serde(rename = "message")]
  message: String,
  /// Whether this task is running or not.
  #[serde(rename = "running")]
  running: bool
}

