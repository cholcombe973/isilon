

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotAliasesExtended {
  #[serde(rename = "aliases")]
  aliases: Option<Vec<::models::SnapshotAliasExtended>>,
  /// Resume token value to use in subsequent calls for continuation.
  #[serde(rename = "resume")]
  resume: Option<String>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

