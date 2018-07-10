

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotLocksExtended {
  #[serde(rename = "locks")]
  locks: Option<Vec<::models::SnapshotLockExtended>>,
  /// Resume token value to use in subsequent calls for continuation.
  #[serde(rename = "resume")]
  resume: Option<String>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

