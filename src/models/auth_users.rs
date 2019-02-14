#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUsers {
    #[serde(rename = "users")]
    pub users: Option<Vec <crate::models::MappingUsersLookupMappingItemUser>>,
}
