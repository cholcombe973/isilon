#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotLocksExtended {
    #[serde(rename = "locks")]
    pub locks: Option<Vec <crate::models::SnapshotLockExtended>>,
    /// Resume token value to use in subsequent calls for continuation.
    #[serde(rename = "resume")]
    pub resume: Option<String>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
