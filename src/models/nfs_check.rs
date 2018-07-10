#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsCheck {
    /// The ID of the export.
    #[serde(rename = "id")]
    pub id: Option<i32>,
    /// The message about the export.
    #[serde(rename = "message")]
    pub message: String,
}
