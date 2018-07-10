
/// AuthRole : Specifies the role maps privileges to users and groups.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRole {
  /// Specifies the description of the role.
  #[serde(rename = "description")]
  description: Option<String>,
  /// Specifies the users or groups that have this role.
  #[serde(rename = "members")]
  members: Option<Vec<::models::AuthAccessAccessItemFileGroup>>,
  /// Specifies the name of the role.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Specifies the privileges granted by this role.
  #[serde(rename = "privileges")]
  privileges: Option<Vec<::models::AuthIdNtokenPrivilegeItem>>
}

