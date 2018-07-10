

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRoles {
  #[serde(rename = "roles")]
  roles: Option<Vec<::models::AuthRoleExtended>>
}

