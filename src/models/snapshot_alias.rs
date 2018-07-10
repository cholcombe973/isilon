

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotAlias {
  /// The user or system supplied snapshot alias name.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Target snapshot for this snapshot alias.
  #[serde(rename = "target")]
  target: Option<String>
}

