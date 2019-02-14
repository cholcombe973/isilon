#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotAliasesExtended {
    #[serde(rename = "aliases")]
    pub aliases: Option<Vec <crate::models::SnapshotAliasExtended>>,
    /// Resume token value to use in subsequent calls for continuation.
    #[serde(rename = "resume")]
    pub resume: Option<String>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
