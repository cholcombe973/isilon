

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotAliasExtended {
  /// The Unix Epoch time the snapshot alias was created.
  #[serde(rename = "created")]
  created: i32,
  /// The system ID given to the snapshot alias.
  #[serde(rename = "id")]
  id: i32,
  /// The user or system supplied snapshot alias name.
  #[serde(rename = "name")]
  name: String,
  /// The ID of the snapshot pointed to.
  #[serde(rename = "target_id")]
  target_id: i32,
  /// The name of the snapshot pointed to.
  #[serde(rename = "target_name")]
  target_name: String
}

