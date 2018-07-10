

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAccessAccessItemShareSharePermissions {
  /// Returns Share level permissions for the user.{ 'read' , 'write' , 'full' or 'none' will be the values}
  #[serde(rename = "expected_permissions")]
  expected_permissions: Option<String>,
  /// Returns whether impersonate guest setting is enabled for the user on the share.
  #[serde(rename = "impersonate_guest")]
  impersonate_guest: Option<bool>,
  /// Returns whether impersonate user setting is enabled on the share
  #[serde(rename = "impersonate_user")]
  impersonate_user: Option<bool>,
  /// Returns whether run as root is enabled for the user on the share
  #[serde(rename = "run_as_root")]
  run_as_root: Option<bool>,
  /// Specifies a list of the relevant Access Control Entries withrespect to the user in the share.
  #[serde(rename = "share_relevant_aces")]
  share_relevant_aces: Option<Vec<::models::AuthAccessAccessItemShareSharePermissionsShareRelevantAce>>
}

