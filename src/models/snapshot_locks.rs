#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotLocks {
    #[serde(rename = "locks")]
    pub locks: Option<Vec<::models::SnapshotLockExtended>>,
}
