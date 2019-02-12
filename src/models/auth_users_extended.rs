#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUsersExtended {
    #[serde(rename = "users")]
    pub users: Option<Vec <crate::models::MappingUsersLookupMappingItemUser>>,
    /// Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options).
    #[serde(rename = "resume")]
    pub resume: Option<String>,
}
