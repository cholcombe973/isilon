#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpUsersExtended {
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
    #[serde(rename = "users")]
    pub users: Option<Vec<::models::NdmpUserExtended>>,
}
