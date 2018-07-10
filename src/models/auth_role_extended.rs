#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRoleExtended {
    /// Specifies the description of the role.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Specifies the users or groups that have this role.
    #[serde(rename = "members")]
    pub members: Vec<::models::AuthAccessAccessItemFileGroup>,
    /// Specifies the name of the role.
    #[serde(rename = "name")]
    pub name: String,
    /// Specifies the privileges granted by this role.
    #[serde(rename = "privileges")]
    pub privileges: Vec<::models::AuthIdNtokenPrivilegeItem>,
    /// Specifies the ID of the role.
    #[serde(rename = "id")]
    pub id: String,
}
