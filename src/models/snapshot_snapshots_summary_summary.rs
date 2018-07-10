

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotSnapshotsSummarySummary {
  /// Total number of active snapshots.
  #[serde(rename = "active_count")]
  active_count: i32,
  /// Sum of sizes of active snapshots.
  #[serde(rename = "active_size")]
  active_size: i32,
  /// Total number of snapshot aliases.
  #[serde(rename = "aliases_count")]
  aliases_count: i32,
  /// Total number of snapshots.
  #[serde(rename = "count")]
  count: i32,
  /// Total number of delete-pending snapshots.
  #[serde(rename = "deleting_count")]
  deleting_count: i32,
  /// Sum of sizes of delete-pending snapshots.
  #[serde(rename = "deleting_size")]
  deleting_size: i32,
  /// Sum of shadow bytes of all snapshots.
  #[serde(rename = "shadow_bytes")]
  shadow_bytes: i32,
  /// Sum of sizes in bytes of all snapshots.
  #[serde(rename = "size")]
  size: i32
}

