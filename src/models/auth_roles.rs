#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRoles {
    #[serde(rename = "roles")]
    pub roles: Option<Vec <crate::models::AuthRoleExtended>>,
}
