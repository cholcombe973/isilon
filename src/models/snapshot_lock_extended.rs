

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotLockExtended {
  /// User supplied lock comment.
  #[serde(rename = "comment")]
  comment: Option<String>,
  /// Recursive lock count.
  #[serde(rename = "count")]
  count: Option<i32>,
  /// The Unix Epoch time the snapshot lock will expire and be eligible for automatic deletion.
  #[serde(rename = "expires")]
  expires: Option<i32>,
  /// System generated lock ID.
  #[serde(rename = "id")]
  id: Option<i32>
}

