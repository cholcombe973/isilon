

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotAliases {
  #[serde(rename = "aliases")]
  aliases: Option<Vec<::models::SnapshotAliasExtended>>
}

