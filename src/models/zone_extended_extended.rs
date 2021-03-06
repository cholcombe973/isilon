#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneExtendedExtended {
    /// Specifies an alternate system provider.
    #[serde(rename = "alternate_system_provider")]
    pub alternate_system_provider: Option<String>,
    /// Specifies the list of authentication providers available on this access zone.
    #[serde(rename = "auth_providers")]
    pub auth_providers: Option<Vec<String>>,
    /// Specifies amount of time in seconds to cache a user/group.
    #[serde(rename = "cache_entry_expiry")]
    pub cache_entry_expiry: Option<i32>,
    /// Determines if a path is created when a path does not exist.
    #[serde(rename = "create_path")]
    pub create_path: Option<bool>,
    /// Groupnet identitier
    #[serde(rename = "groupnet")]
    pub groupnet: Option<String>,
    /// Specifies the permissions set on automatically created user home directories.
    #[serde(rename = "home_directory_umask")]
    pub home_directory_umask: Option<i32>,
    /// Specifies the system-assigned ID for the access zone. This value is returned when an access zone is created through the POST method
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Specifies a list of users and groups that have read and write access to /ifs.
    #[serde(rename = "ifs_restricted")]
    pub ifs_restricted: Option<Vec<crate::models::AuthAccessAccessItemFileGroup>>,
    /// Maps untrusted domains to this NetBIOS domain during authentication.
    #[serde(rename = "map_untrusted")]
    pub map_untrusted: Option<String>,
    /// Specifies the access zone name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Specifies number of seconds the negative cache entry is valid.
    #[serde(rename = "negative_cache_entry_expiry")]
    pub negative_cache_entry_expiry: Option<i32>,
    /// Specifies the NetBIOS name.
    #[serde(rename = "netbios_name")]
    pub netbios_name: Option<String>,
    /// Specifies the access zone base directory path.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// Specifies the skeleton directory that is used for user home directories.
    #[serde(rename = "skeleton_directory")]
    pub skeleton_directory: Option<String>,
    /// True if the access zone is built-in.
    #[serde(rename = "system")]
    pub system: Option<bool>,
    /// Specifies the system provider for the access zone.
    #[serde(rename = "system_provider")]
    pub system_provider: Option<String>,
    /// Specifies the current ID mapping rules.
    #[serde(rename = "user_mapping_rules")]
    pub user_mapping_rules: Option<Vec<String>>,
    /// Specifies the access zone ID on the system.
    #[serde(rename = "zone_id")]
    pub zone_id: Option<i32>,
}
