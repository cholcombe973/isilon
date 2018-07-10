#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSnapshotAliasResponse {
    /// The ID of the newly created snapshot alias.
    #[serde(rename = "id")]
    pub id: i32,
}
