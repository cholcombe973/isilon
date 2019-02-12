#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotAliases {
    #[serde(rename = "aliases")]
    pub aliases: Option<Vec<crate::models::SnapshotAliasExtended>>,
}
