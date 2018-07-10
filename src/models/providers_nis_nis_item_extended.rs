#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersNisNisItemExtended {
    /// If true, enables authentication and identity management through the authentication provider.
    #[serde(rename = "authentication")]
    pub authentication: Option<bool>,
    /// If true, connects the provider to a random server.
    #[serde(rename = "balance_servers")]
    pub balance_servers: Option<bool>,
    /// Specifies the time in seconds between provider online checks.
    #[serde(rename = "check_online_interval")]
    pub check_online_interval: Option<i32>,
    /// Automatically creates the home directory on the first login.
    #[serde(rename = "create_home_directory")]
    pub create_home_directory: Option<bool>,
    /// If true, enables the NIS provider.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// If true, allows the provider to enumerate groups.
    #[serde(rename = "enumerate_groups")]
    pub enumerate_groups: Option<bool>,
    /// If true, allows the provider to enumerate users.
    #[serde(rename = "enumerate_users")]
    pub enumerate_users: Option<bool>,
    /// Specifies the list of groups that can be resolved.
    #[serde(rename = "findable_groups")]
    pub findable_groups: Option<Vec<String>>,
    /// Specifies the list of users that can be resolved.
    #[serde(rename = "findable_users")]
    pub findable_users: Option<Vec<String>>,
    /// Specifies the domain for this provider through which groups are qualified.
    #[serde(rename = "group_domain")]
    pub group_domain: Option<String>,
    /// Specifies the path to the home directory template.
    #[serde(rename = "home_directory_template")]
    pub home_directory_template: Option<String>,
    /// If true, enables host name look ups.
    #[serde(rename = "hostname_lookup")]
    pub hostname_lookup: Option<bool>,
    /// Specifies the NIS provider ID.
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
    /// Specifies the NIS provider name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Specifies the NIS domain name.
    #[serde(rename = "nis_domain")]
    pub nis_domain: Option<String>,
    /// Normalizes group names to lowercase before look up.
    #[serde(rename = "normalize_groups")]
    pub normalize_groups: Option<bool>,
    /// Normalizes user names to lowercase before look up.
    #[serde(rename = "normalize_users")]
    pub normalize_users: Option<bool>,
    /// Specifies which NTLM versions to support for users with NTLM-compatible credentials.
    #[serde(rename = "ntlm_support")]
    pub ntlm_support: Option<String>,
    /// Specifies the domain for the provider.
    #[serde(rename = "provider_domain")]
    pub provider_domain: Option<String>,
    /// Specifies the request timeout interval in seconds.
    #[serde(rename = "request_timeout")]
    pub request_timeout: Option<i32>,
    /// If true, checks the provider for filtered lists of findable and unfindable users and groups.
    #[serde(rename = "restrict_findable")]
    pub restrict_findable: Option<bool>,
    /// If true, checks the provider for filtered lists of listable and unlistable users and groups.
    #[serde(rename = "restrict_listable")]
    pub restrict_listable: Option<bool>,
    /// Specifies the timeout period in seconds after which a request will be retried.
    #[serde(rename = "retry_time")]
    pub retry_time: Option<i32>,
    /// Adds an NIS server for this provider.
    #[serde(rename = "servers")]
    pub servers: Option<Vec<String>>,
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
    /// Specifies the domain for this provider through which users are qualified.
    #[serde(rename = "user_domain")]
    pub user_domain: Option<String>,
    /// If true, specifies TCP for YP Match operations.
    #[serde(rename = "ypmatch_using_tcp")]
    pub ypmatch_using_tcp: Option<bool>,
    /// Groupnet identifier.
    #[serde(rename = "groupnet")]
    pub groupnet: Option<String>,
}
