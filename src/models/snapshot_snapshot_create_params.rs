

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotSnapshotCreateParams {
  /// Alias name to create for this snapshot. If null, remove any alias.
  #[serde(rename = "alias")]
  alias: Option<String>,
  /// The Unix Epoch time the snapshot will expire and be eligible for automatic deletion.
  #[serde(rename = "expires")]
  expires: Option<i32>,
  /// The user or system supplied snapshot name. This will be null for snapshots pending delete.
  #[serde(rename = "name")]
  name: Option<String>,
  /// The /ifs path snapshotted.
  #[serde(rename = "path")]
  path: String
}

