

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUsers {
  #[serde(rename = "users")]
  users: Option<Vec<::models::MappingUsersLookupMappingItemUser>>
}

