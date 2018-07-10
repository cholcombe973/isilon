/// ProvidersAdsItem : Specifies the properties for an ADS authentication provider.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersAdsItem {
    /// Specifies the machine account name when creating a SAM account with Active Directory. The default cluster name is called 'default'.
    #[serde(rename = "account")]
    pub account: Option<String>,
    /// Allocates an ID for an unmapped Active Directory (ADS) group. ADS groups without GIDs can be proactively assigned a GID by the ID mapper. If the ID mapper option is disabled, GIDs are not proactively assigned, and when a primary group for a user does not include a GID, the system may allocate one.
    #[serde(rename = "allocate_gids")]
    pub allocate_gids: Option<bool>,
    /// Allocates a user ID for an unmapped Active Directory (ADS) user. ADS users without UIDs can be proactively assigned a UID by the ID mapper. IF the ID mapper option is disabled, UIDs are not proactively assigned, and when an identify for a user does not include a UID, the system may allocate one.
    #[serde(rename = "allocate_uids")]
    pub allocate_uids: Option<bool>,
    /// Enables lookup of unqualified user names in the primary domain.
    #[serde(rename = "assume_default_domain")]
    pub assume_default_domain: Option<bool>,
    /// Enables authentication and identity management through the authentication provider.
    #[serde(rename = "authentication")]
    pub authentication: Option<bool>,
    /// Specifies the time in seconds between provider online checks.
    #[serde(rename = "check_online_interval")]
    pub check_online_interval: Option<i32>,
    /// Specifies the current time for the domain controllers.
    #[serde(rename = "controller_time")]
    pub controller_time: Option<i32>,
    /// Automatically creates a home directory on the first login.
    #[serde(rename = "create_home_directory")]
    pub create_home_directory: Option<bool>,
    /// Specifies the DNS search domain. Set this parameter if the DNS search domain has a unique name or address.
    #[serde(rename = "dns_domain")]
    pub dns_domain: Option<String>,
    /// Sends an alert if the domain goes offline.
    #[serde(rename = "domain_offline_alerts")]
    pub domain_offline_alerts: Option<bool>,
    /// Sets list of groups that can be resolved.
    #[serde(rename = "findable_groups")]
    pub findable_groups: Option<Vec<String>>,
    /// Sets list of users that can be resolved.
    #[serde(rename = "findable_users")]
    pub findable_users: Option<Vec<String>>,
    /// Groupnet identifier.
    #[serde(rename = "groupnet")]
    pub groupnet: Option<String>,
    /// Specifies the path to the home directory template.
    #[serde(rename = "home_directory_template")]
    pub home_directory_template: Option<String>,
    /// If set to true, ignores all trusted domains.
    #[serde(rename = "ignore_all_trusts")]
    pub ignore_all_trusts: Option<bool>,
    /// Includes trusted domains when 'ignore_all_trusts' is set to false.
    #[serde(rename = "ignored_trusted_domains")]
    pub ignored_trusted_domains: Option<Vec<String>>,
    /// Includes trusted domains when 'ignore_all_trusts' is set to true.
    #[serde(rename = "include_trusted_domains")]
    pub include_trusted_domains: Option<Vec<String>>,
    /// Specifies Active Directory provider instance.
    #[serde(rename = "instance")]
    pub instance: Option<String>,
    /// Determines if connecting through HDFS with Kerberos.
    #[serde(rename = "kerberos_hdfs_spn")]
    pub kerberos_hdfs_spn: Option<bool>,
    /// Determines if connecting through NFS with Kerberos.
    #[serde(rename = "kerberos_nfs_spn")]
    pub kerberos_nfs_spn: Option<bool>,
    /// Enables encryption and signing on LDAP requests.
    #[serde(rename = "ldap_sign_and_seal")]
    pub ldap_sign_and_seal: Option<bool>,
    /// Specifies the login shell path.
    #[serde(rename = "login_shell")]
    pub login_shell: Option<String>,
    /// Limits user and group lookups to the specified domains.
    #[serde(rename = "lookup_domains")]
    pub lookup_domains: Option<Vec<String>>,
    /// Looks up AD groups in other providers before allocating a group ID.
    #[serde(rename = "lookup_groups")]
    pub lookup_groups: Option<bool>,
    /// Normalizes AD group names to lowercase before look up.
    #[serde(rename = "lookup_normalize_groups")]
    pub lookup_normalize_groups: Option<bool>,
    /// Normalize AD user names to lowercase before look up.
    #[serde(rename = "lookup_normalize_users")]
    pub lookup_normalize_users: Option<bool>,
    /// Looks up AD users in other providers before allocating a user ID.
    #[serde(rename = "lookup_users")]
    pub lookup_users: Option<bool>,
    /// Specifies name to join AD as.
    #[serde(rename = "machine_name")]
    pub machine_name: Option<String>,
    /// Enables periodic changes of the machine password for security.
    #[serde(rename = "machine_password_changes")]
    pub machine_password_changes: Option<bool>,
    /// Sets maximum age of a password in seconds.
    #[serde(rename = "machine_password_lifespan")]
    pub machine_password_lifespan: Option<i32>,
    /// Specifies the Active Directory provider name.
    #[serde(rename = "name")]
    pub name: String,
    /// Specifies the domain controller for which the node has affinity.
    #[serde(rename = "node_dc_affinity")]
    pub node_dc_affinity: Option<String>,
    /// Specifies the timeout for the domain controller for which the local node has affinity.
    #[serde(rename = "node_dc_affinity_timeout")]
    pub node_dc_affinity_timeout: Option<i32>,
    /// Enables the Active Directory provider to respond to 'getpwent' and 'getgrent' requests.
    #[serde(rename = "nss_enumeration")]
    pub nss_enumeration: Option<bool>,
    /// Specifies the organizational unit.
    #[serde(rename = "organizational_unit")]
    pub organizational_unit: Option<String>,
    /// Specifies the password used during domain join.
    #[serde(rename = "password")]
    pub password: String,
    /// Check the provider for filtered lists of findable and unfindable users and groups.
    #[serde(rename = "restrict_findable")]
    pub restrict_findable: Option<bool>,
    /// Specifies whether to support RFC 2307 attributes on ADS domain controllers.
    #[serde(rename = "sfu_support")]
    pub sfu_support: Option<String>,
    /// Stores SFU mappings permanently in the ID mapper.
    #[serde(rename = "store_sfu_mappings")]
    pub store_sfu_mappings: Option<bool>,
    /// Specifies groups that cannot be resolved by the provider.
    #[serde(rename = "unfindable_groups")]
    pub unfindable_groups: Option<Vec<String>>,
    /// Specifies users that cannot be resolved by the provider.
    #[serde(rename = "unfindable_users")]
    pub unfindable_users: Option<Vec<String>>,
    /// Specifies the user name that has permission to join a machine to the given domain.
    #[serde(rename = "user")]
    pub user: String,
}
