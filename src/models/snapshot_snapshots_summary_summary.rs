#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotSnapshotsSummarySummary {
    /// Total number of active snapshots.
    #[serde(rename = "active_count")]
    pub active_count: i32,
    /// Sum of sizes of active snapshots.
    #[serde(rename = "active_size")]
    pub active_size: i32,
    /// Total number of snapshot aliases.
    #[serde(rename = "aliases_count")]
    pub aliases_count: i32,
    /// Total number of snapshots.
    #[serde(rename = "count")]
    pub count: i32,
    /// Total number of delete-pending snapshots.
    #[serde(rename = "deleting_count")]
    pub deleting_count: i32,
    /// Sum of sizes of delete-pending snapshots.
    #[serde(rename = "deleting_size")]
    pub deleting_size: i32,
    /// Sum of shadow bytes of all snapshots.
    #[serde(rename = "shadow_bytes")]
    pub shadow_bytes: u64,
    /// Sum of sizes in bytes of all snapshots.
    #[serde(rename = "size")]
    pub size: u64,
}
