#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpUsers {
    #[serde(rename = "users")]
    pub users: Option<Vec<::models::NdmpUserExtended>>,
}
