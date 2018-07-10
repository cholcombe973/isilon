

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersLookupMappingItemUser {
  #[serde(rename = "dn")]
  dn: Option<String>,
  #[serde(rename = "dns_domain")]
  dns_domain: Option<String>,
  #[serde(rename = "domain")]
  domain: Option<String>,
  #[serde(rename = "email")]
  email: Option<String>,
  /// True, if the authenticated user is enabled.
  #[serde(rename = "enabled")]
  enabled: bool,
  /// True, if the authenticated user has expired.
  #[serde(rename = "expired")]
  expired: bool,
  #[serde(rename = "expiry")]
  expiry: Option<i32>,
  #[serde(rename = "gecos")]
  gecos: Option<String>,
  /// True, if the GID was generated.
  #[serde(rename = "generated_gid")]
  generated_gid: Option<bool>,
  /// True, if the UID was generated.
  #[serde(rename = "generated_uid")]
  generated_uid: Option<bool>,
  /// True, if the UPN was generated.
  #[serde(rename = "generated_upn")]
  generated_upn: Option<bool>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "gid")]
  gid: Option<::models::AuthAccessAccessItemFileGroup>,
  #[serde(rename = "home_directory")]
  home_directory: Option<String>,
  /// Specifies the user or group ID.
  #[serde(rename = "id")]
  id: String,
  /// If true, indicates that the account is locked.
  #[serde(rename = "locked")]
  locked: bool,
  /// Specifies the maximum time in seconds allowed before the password expires.
  #[serde(rename = "max_password_age")]
  max_password_age: Option<i32>,
  #[serde(rename = "member_of")]
  member_of: Option<Vec<::models::AuthAccessAccessItemFileGroup>>,
  /// Specifies a user or group name.
  #[serde(rename = "name")]
  name: String,
  #[serde(rename = "object_history")]
  object_history: Option<Vec<::models::AuthGroupObjectHistoryItem>>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "on_disk_group_identity")]
  on_disk_group_identity: Option<::models::AuthAccessAccessItemFileGroup>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "on_disk_user_identity")]
  on_disk_user_identity: Option<::models::AuthAccessAccessItemFileGroup>,
  /// If true, the password has expired.
  #[serde(rename = "password_expired")]
  password_expired: bool,
  /// If true, the password is allowed to expire.
  #[serde(rename = "password_expires")]
  password_expires: bool,
  #[serde(rename = "password_expiry")]
  password_expiry: Option<i32>,
  #[serde(rename = "password_last_set")]
  password_last_set: Option<i32>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "primary_group_sid")]
  primary_group_sid: Option<::models::AuthAccessAccessItemFileGroup>,
  /// Prompts the user to change their password at the next login.
  #[serde(rename = "prompt_password_change")]
  prompt_password_change: bool,
  #[serde(rename = "provider")]
  provider: Option<String>,
  #[serde(rename = "sam_account_name")]
  sam_account_name: Option<String>,
  #[serde(rename = "shell")]
  shell: Option<String>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "sid")]
  sid: Option<::models::AuthAccessAccessItemFileGroup>,
  /// Specifies the object type.
  #[serde(rename = "type")]
  _type: String,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "uid")]
  uid: Option<::models::AuthAccessAccessItemFileGroup>,
  #[serde(rename = "upn")]
  upn: Option<String>,
  /// Specifies whether the password for the user can be changed.
  #[serde(rename = "user_can_change_password")]
  user_can_change_password: bool
}

