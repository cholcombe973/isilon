

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneExtendedExtended {
  /// Specifies an alternate system provider.
  #[serde(rename = "alternate_system_provider")]
  alternate_system_provider: Option<String>,
  /// Specifies the list of authentication providers available on this access zone.
  #[serde(rename = "auth_providers")]
  auth_providers: Option<Vec<String>>,
  /// Specifies amount of time in seconds to cache a user/group.
  #[serde(rename = "cache_entry_expiry")]
  cache_entry_expiry: Option<i32>,
  /// Determines if a path is created when a path does not exist.
  #[serde(rename = "create_path")]
  create_path: Option<bool>,
  /// Groupnet identitier
  #[serde(rename = "groupnet")]
  groupnet: Option<String>,
  /// Specifies the permissions set on automatically created user home directories.
  #[serde(rename = "home_directory_umask")]
  home_directory_umask: Option<i32>,
  /// Specifies the system-assigned ID for the access zone. This value is returned when an access zone is created through the POST method
  #[serde(rename = "id")]
  id: Option<String>,
  /// Specifies a list of users and groups that have read and write access to /ifs.
  #[serde(rename = "ifs_restricted")]
  ifs_restricted: Option<Vec<::models::AuthAccessAccessItemFileGroup>>,
  /// Maps untrusted domains to this NetBIOS domain during authentication.
  #[serde(rename = "map_untrusted")]
  map_untrusted: Option<String>,
  /// Specifies the access zone name.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Specifies number of seconds the negative cache entry is valid.
  #[serde(rename = "negative_cache_entry_expiry")]
  negative_cache_entry_expiry: Option<i32>,
  /// Specifies the NetBIOS name.
  #[serde(rename = "netbios_name")]
  netbios_name: Option<String>,
  /// Specifies the access zone base directory path.
  #[serde(rename = "path")]
  path: Option<String>,
  /// Specifies the skeleton directory that is used for user home directories.
  #[serde(rename = "skeleton_directory")]
  skeleton_directory: Option<String>,
  /// True if the access zone is built-in.
  #[serde(rename = "system")]
  system: Option<bool>,
  /// Specifies the system provider for the access zone.
  #[serde(rename = "system_provider")]
  system_provider: Option<String>,
  /// Specifies the current ID mapping rules.
  #[serde(rename = "user_mapping_rules")]
  user_mapping_rules: Option<Vec<String>>,
  /// Specifies the access zone ID on the system.
  #[serde(rename = "zone_id")]
  zone_id: Option<i32>
}

