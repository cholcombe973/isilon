

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSnapshotLockResponse {
  /// The ID of the newly created snapshot lock.
  #[serde(rename = "id")]
  id: i32
}

