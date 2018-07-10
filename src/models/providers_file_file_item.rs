#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersFileFileItem {
    /// Enables authentication and identity mapping through the authentication provider.
    #[serde(rename = "authentication")]
    pub authentication: Option<bool>,
    /// Automatically creates a home directory on the first login.
    #[serde(rename = "create_home_directory")]
    pub create_home_directory: Option<bool>,
    /// Enables the file provider.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Enables the provider to enumerate groups.
    #[serde(rename = "enumerate_groups")]
    pub enumerate_groups: Option<bool>,
    /// Enables the provider to enumerate users.
    #[serde(rename = "enumerate_users")]
    pub enumerate_users: Option<bool>,
    /// Specifies the list of groups that can be resolved.
    #[serde(rename = "findable_groups")]
    pub findable_groups: Option<Vec<String>>,
    /// Specifies the list of users that can be resolved.
    #[serde(rename = "findable_users")]
    pub findable_users: Option<Vec<String>>,
    /// Specifies the domain for this provider through which domains are qualified.
    #[serde(rename = "group_domain")]
    pub group_domain: Option<String>,
    /// Specifies the location of the file that contains information about the group.
    #[serde(rename = "group_file")]
    pub group_file: Option<String>,
    /// Specifies the path to the home directory template.
    #[serde(rename = "home_directory_template")]
    pub home_directory_template: Option<String>,
    /// Specifies the file provider ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Specifies the groups that can be viewed in the provider.
    #[serde(rename = "listable_groups")]
    pub listable_groups: Option<Vec<String>>,
    /// Specifies the users that can be viewed in the provider.
    #[serde(rename = "listable_users")]
    pub listable_users: Option<Vec<String>>,
    /// Specifies the login shell path.
    #[serde(rename = "login_shell")]
    pub login_shell: Option<String>,
    /// Specifies the groups that can be modified in the provider.
    #[serde(rename = "modifiable_groups")]
    pub modifiable_groups: Option<Vec<String>>,
    /// Specifies the users that can be modified in the provider.
    #[serde(rename = "modifiable_users")]
    pub modifiable_users: Option<Vec<String>>,
    /// Specifies the name of the file provider.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Specifies the path to a netgroups replacement file.
    #[serde(rename = "netgroup_file")]
    pub netgroup_file: Option<String>,
    /// Normalizes group names to lowercase before look up.
    #[serde(rename = "normalize_groups")]
    pub normalize_groups: Option<bool>,
    /// Normalizes user names to lowercase before look up.
    #[serde(rename = "normalize_users")]
    pub normalize_users: Option<bool>,
    /// Specifies which NTLM versions to support for users with NTLM-compatible credentials.
    #[serde(rename = "ntlm_support")]
    pub ntlm_support: Option<String>,
    /// Specifies the location of the file containing information about users.
    #[serde(rename = "password_file")]
    pub password_file: Option<String>,
    /// Specifies the domain for the provider.
    #[serde(rename = "provider_domain")]
    pub provider_domain: Option<String>,
    /// If true, checks the provider for filtered lists of findable and unfindable users and groups.
    #[serde(rename = "restrict_findable")]
    pub restrict_findable: Option<bool>,
    /// If true, checks the provider for filtered lists of listable and unlistable users and groups.
    #[serde(rename = "restrict_listable")]
    pub restrict_listable: Option<bool>,
    /// If true, checks the provider for filtered lists of modifiable and unmodifiable users and groups.
    #[serde(rename = "restrict_modifiable")]
    pub restrict_modifiable: Option<bool>,
    /// Specifies the status of the provider.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// If true, indicates that this provider instance was created by OneFS and cannot be removed.
    #[serde(rename = "system")]
    pub system: Option<bool>,
    /// Specifies groups that cannot be resolved by the provider.
    #[serde(rename = "unfindable_groups")]
    pub unfindable_groups: Option<Vec<String>>,
    /// Specifies users that cannot be resolved by the provider.
    #[serde(rename = "unfindable_users")]
    pub unfindable_users: Option<Vec<String>>,
    /// Specifies a group that cannot be listed by the provider.
    #[serde(rename = "unlistable_groups")]
    pub unlistable_groups: Option<Vec<String>>,
    /// Specifies a user that cannot be listed by the provider.
    #[serde(rename = "unlistable_users")]
    pub unlistable_users: Option<Vec<String>>,
    /// Specifies a group that cannot be modified by the provider.
    #[serde(rename = "unmodifiable_groups")]
    pub unmodifiable_groups: Option<Vec<String>>,
    /// Specifies a user that cannot be modified by the provider.
    #[serde(rename = "unmodifiable_users")]
    pub unmodifiable_users: Option<Vec<String>>,
    /// Specifies the domain for this provider through which users are qualified.
    #[serde(rename = "user_domain")]
    pub user_domain: Option<String>,
}
