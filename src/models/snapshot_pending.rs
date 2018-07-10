

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotPending {
  #[serde(rename = "pending")]
  pending: Option<Vec<::models::SnapshotPendingPendingItem>>,
  /// Resume token value to use in subsequent calls for continuation.
  #[serde(rename = "resume")]
  resume: Option<String>
}

