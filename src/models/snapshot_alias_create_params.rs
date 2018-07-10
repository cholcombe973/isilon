#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotAliasCreateParams {
    /// The user or system supplied snapshot name.
    #[serde(rename = "name")]
    pub name: String,
    /// Snapshot name target for the alias.
    #[serde(rename = "target")]
    pub target: String,
}
