
/// ProvidersFileIdParams : Specifies the properties for an authentication file provider

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersFileIdParams {
  /// Enables authentication and identity mapping through the authentication provider.
  #[serde(rename = "authentication")]
  authentication: Option<bool>,
  /// Automatically creates a home directory on the first login.
  #[serde(rename = "create_home_directory")]
  create_home_directory: Option<bool>,
  /// Enables the file provider.
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  /// Enables the provider to enumerate groups.
  #[serde(rename = "enumerate_groups")]
  enumerate_groups: Option<bool>,
  /// Enables the provider to enumerate users.
  #[serde(rename = "enumerate_users")]
  enumerate_users: Option<bool>,
  /// Specifies the list of groups that can be resolved.
  #[serde(rename = "findable_groups")]
  findable_groups: Option<Vec<String>>,
  /// Specifies the list of users that can be resolved.
  #[serde(rename = "findable_users")]
  findable_users: Option<Vec<String>>,
  /// Specifies the domain for this provider through which domains are qualified.
  #[serde(rename = "group_domain")]
  group_domain: Option<String>,
  /// Specifies the location of the file that contains information about the group.
  #[serde(rename = "group_file")]
  group_file: Option<String>,
  /// Specifies the path to the home directory template.
  #[serde(rename = "home_directory_template")]
  home_directory_template: Option<String>,
  /// Specifies the groups that can be viewed in the provider.
  #[serde(rename = "listable_groups")]
  listable_groups: Option<Vec<String>>,
  /// Specifies the users that can be viewed in the provider.
  #[serde(rename = "listable_users")]
  listable_users: Option<Vec<String>>,
  /// Specifies the login shell path.
  #[serde(rename = "login_shell")]
  login_shell: Option<String>,
  /// Specifies the groups that can be modified in the provider.
  #[serde(rename = "modifiable_groups")]
  modifiable_groups: Option<Vec<String>>,
  /// Specifies the users that can be modified in the provider.
  #[serde(rename = "modifiable_users")]
  modifiable_users: Option<Vec<String>>,
  /// Specifies the name of the file provider.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Specifies the path to a netgroups replacement file.
  #[serde(rename = "netgroup_file")]
  netgroup_file: Option<String>,
  /// Normalizes group names to lowercase before look up.
  #[serde(rename = "normalize_groups")]
  normalize_groups: Option<bool>,
  /// Normalizes user names to lowercase before look up.
  #[serde(rename = "normalize_users")]
  normalize_users: Option<bool>,
  /// Specifies which NTLM versions to support for users with NTLM-compatible credentials.
  #[serde(rename = "ntlm_support")]
  ntlm_support: Option<String>,
  /// Specifies the location of the file containing information about users.
  #[serde(rename = "password_file")]
  password_file: Option<String>,
  /// Specifies the domain for the provider.
  #[serde(rename = "provider_domain")]
  provider_domain: Option<String>,
  /// If true, checks the provider for filtered lists of findable and unfindable users and groups.
  #[serde(rename = "restrict_findable")]
  restrict_findable: Option<bool>,
  /// If true, checks the provider for filtered lists of listable and unlistable users and groups.
  #[serde(rename = "restrict_listable")]
  restrict_listable: Option<bool>,
  /// If true, checks the provider for filtered lists of modifiable and unmodifiable users and groups.
  #[serde(rename = "restrict_modifiable")]
  restrict_modifiable: Option<bool>,
  /// Specifies groups that cannot be resolved by the provider.
  #[serde(rename = "unfindable_groups")]
  unfindable_groups: Option<Vec<String>>,
  /// Specifies users that cannot be resolved by the provider.
  #[serde(rename = "unfindable_users")]
  unfindable_users: Option<Vec<String>>,
  /// Specifies a group that cannot be listed by the provider.
  #[serde(rename = "unlistable_groups")]
  unlistable_groups: Option<Vec<String>>,
  /// Specifies a user that cannot be listed by the provider.
  #[serde(rename = "unlistable_users")]
  unlistable_users: Option<Vec<String>>,
  /// Specifies a group that cannot be modified by the provider.
  #[serde(rename = "unmodifiable_groups")]
  unmodifiable_groups: Option<Vec<String>>,
  /// Specifies a user that cannot be modified by the provider.
  #[serde(rename = "unmodifiable_users")]
  unmodifiable_users: Option<Vec<String>>,
  /// Specifies the domain for this provider through which users are qualified.
  #[serde(rename = "user_domain")]
  user_domain: Option<String>
}

