

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotLocks {
  #[serde(rename = "locks")]
  locks: Option<Vec<::models::SnapshotLockExtended>>
}

