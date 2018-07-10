/// CreateSyncReportsRotateItemResponse : Force rotation of reports.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSyncReportsRotateItemResponse {
    /// A message about the status of the task.
    #[serde(rename = "message")]
    pub message: String,
}
