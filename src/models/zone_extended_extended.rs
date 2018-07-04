/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


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

impl ZoneExtendedExtended {
  pub fn new() -> ZoneExtendedExtended {
    ZoneExtendedExtended {
      alternate_system_provider: None,
      auth_providers: None,
      cache_entry_expiry: None,
      create_path: None,
      groupnet: None,
      home_directory_umask: None,
      id: None,
      ifs_restricted: None,
      map_untrusted: None,
      name: None,
      negative_cache_entry_expiry: None,
      netbios_name: None,
      path: None,
      skeleton_directory: None,
      system: None,
      system_provider: None,
      user_mapping_rules: None,
      zone_id: None
    }
  }

  pub fn set_alternate_system_provider(&mut self, alternate_system_provider: String) {
    self.alternate_system_provider = Some(alternate_system_provider);
  }

  pub fn with_alternate_system_provider(mut self, alternate_system_provider: String) -> ZoneExtendedExtended {
    self.alternate_system_provider = Some(alternate_system_provider);
    self
  }

  pub fn alternate_system_provider(&self) -> Option<&String> {
    self.alternate_system_provider.as_ref()
  }

  pub fn reset_alternate_system_provider(&mut self) {
    self.alternate_system_provider = None;
  }

  pub fn set_auth_providers(&mut self, auth_providers: Vec<String>) {
    self.auth_providers = Some(auth_providers);
  }

  pub fn with_auth_providers(mut self, auth_providers: Vec<String>) -> ZoneExtendedExtended {
    self.auth_providers = Some(auth_providers);
    self
  }

  pub fn auth_providers(&self) -> Option<&Vec<String>> {
    self.auth_providers.as_ref()
  }

  pub fn reset_auth_providers(&mut self) {
    self.auth_providers = None;
  }

  pub fn set_cache_entry_expiry(&mut self, cache_entry_expiry: i32) {
    self.cache_entry_expiry = Some(cache_entry_expiry);
  }

  pub fn with_cache_entry_expiry(mut self, cache_entry_expiry: i32) -> ZoneExtendedExtended {
    self.cache_entry_expiry = Some(cache_entry_expiry);
    self
  }

  pub fn cache_entry_expiry(&self) -> Option<&i32> {
    self.cache_entry_expiry.as_ref()
  }

  pub fn reset_cache_entry_expiry(&mut self) {
    self.cache_entry_expiry = None;
  }

  pub fn set_create_path(&mut self, create_path: bool) {
    self.create_path = Some(create_path);
  }

  pub fn with_create_path(mut self, create_path: bool) -> ZoneExtendedExtended {
    self.create_path = Some(create_path);
    self
  }

  pub fn create_path(&self) -> Option<&bool> {
    self.create_path.as_ref()
  }

  pub fn reset_create_path(&mut self) {
    self.create_path = None;
  }

  pub fn set_groupnet(&mut self, groupnet: String) {
    self.groupnet = Some(groupnet);
  }

  pub fn with_groupnet(mut self, groupnet: String) -> ZoneExtendedExtended {
    self.groupnet = Some(groupnet);
    self
  }

  pub fn groupnet(&self) -> Option<&String> {
    self.groupnet.as_ref()
  }

  pub fn reset_groupnet(&mut self) {
    self.groupnet = None;
  }

  pub fn set_home_directory_umask(&mut self, home_directory_umask: i32) {
    self.home_directory_umask = Some(home_directory_umask);
  }

  pub fn with_home_directory_umask(mut self, home_directory_umask: i32) -> ZoneExtendedExtended {
    self.home_directory_umask = Some(home_directory_umask);
    self
  }

  pub fn home_directory_umask(&self) -> Option<&i32> {
    self.home_directory_umask.as_ref()
  }

  pub fn reset_home_directory_umask(&mut self) {
    self.home_directory_umask = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> ZoneExtendedExtended {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_ifs_restricted(&mut self, ifs_restricted: Vec<::models::AuthAccessAccessItemFileGroup>) {
    self.ifs_restricted = Some(ifs_restricted);
  }

  pub fn with_ifs_restricted(mut self, ifs_restricted: Vec<::models::AuthAccessAccessItemFileGroup>) -> ZoneExtendedExtended {
    self.ifs_restricted = Some(ifs_restricted);
    self
  }

  pub fn ifs_restricted(&self) -> Option<&Vec<::models::AuthAccessAccessItemFileGroup>> {
    self.ifs_restricted.as_ref()
  }

  pub fn reset_ifs_restricted(&mut self) {
    self.ifs_restricted = None;
  }

  pub fn set_map_untrusted(&mut self, map_untrusted: String) {
    self.map_untrusted = Some(map_untrusted);
  }

  pub fn with_map_untrusted(mut self, map_untrusted: String) -> ZoneExtendedExtended {
    self.map_untrusted = Some(map_untrusted);
    self
  }

  pub fn map_untrusted(&self) -> Option<&String> {
    self.map_untrusted.as_ref()
  }

  pub fn reset_map_untrusted(&mut self) {
    self.map_untrusted = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> ZoneExtendedExtended {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_negative_cache_entry_expiry(&mut self, negative_cache_entry_expiry: i32) {
    self.negative_cache_entry_expiry = Some(negative_cache_entry_expiry);
  }

  pub fn with_negative_cache_entry_expiry(mut self, negative_cache_entry_expiry: i32) -> ZoneExtendedExtended {
    self.negative_cache_entry_expiry = Some(negative_cache_entry_expiry);
    self
  }

  pub fn negative_cache_entry_expiry(&self) -> Option<&i32> {
    self.negative_cache_entry_expiry.as_ref()
  }

  pub fn reset_negative_cache_entry_expiry(&mut self) {
    self.negative_cache_entry_expiry = None;
  }

  pub fn set_netbios_name(&mut self, netbios_name: String) {
    self.netbios_name = Some(netbios_name);
  }

  pub fn with_netbios_name(mut self, netbios_name: String) -> ZoneExtendedExtended {
    self.netbios_name = Some(netbios_name);
    self
  }

  pub fn netbios_name(&self) -> Option<&String> {
    self.netbios_name.as_ref()
  }

  pub fn reset_netbios_name(&mut self) {
    self.netbios_name = None;
  }

  pub fn set_path(&mut self, path: String) {
    self.path = Some(path);
  }

  pub fn with_path(mut self, path: String) -> ZoneExtendedExtended {
    self.path = Some(path);
    self
  }

  pub fn path(&self) -> Option<&String> {
    self.path.as_ref()
  }

  pub fn reset_path(&mut self) {
    self.path = None;
  }

  pub fn set_skeleton_directory(&mut self, skeleton_directory: String) {
    self.skeleton_directory = Some(skeleton_directory);
  }

  pub fn with_skeleton_directory(mut self, skeleton_directory: String) -> ZoneExtendedExtended {
    self.skeleton_directory = Some(skeleton_directory);
    self
  }

  pub fn skeleton_directory(&self) -> Option<&String> {
    self.skeleton_directory.as_ref()
  }

  pub fn reset_skeleton_directory(&mut self) {
    self.skeleton_directory = None;
  }

  pub fn set_system(&mut self, system: bool) {
    self.system = Some(system);
  }

  pub fn with_system(mut self, system: bool) -> ZoneExtendedExtended {
    self.system = Some(system);
    self
  }

  pub fn system(&self) -> Option<&bool> {
    self.system.as_ref()
  }

  pub fn reset_system(&mut self) {
    self.system = None;
  }

  pub fn set_system_provider(&mut self, system_provider: String) {
    self.system_provider = Some(system_provider);
  }

  pub fn with_system_provider(mut self, system_provider: String) -> ZoneExtendedExtended {
    self.system_provider = Some(system_provider);
    self
  }

  pub fn system_provider(&self) -> Option<&String> {
    self.system_provider.as_ref()
  }

  pub fn reset_system_provider(&mut self) {
    self.system_provider = None;
  }

  pub fn set_user_mapping_rules(&mut self, user_mapping_rules: Vec<String>) {
    self.user_mapping_rules = Some(user_mapping_rules);
  }

  pub fn with_user_mapping_rules(mut self, user_mapping_rules: Vec<String>) -> ZoneExtendedExtended {
    self.user_mapping_rules = Some(user_mapping_rules);
    self
  }

  pub fn user_mapping_rules(&self) -> Option<&Vec<String>> {
    self.user_mapping_rules.as_ref()
  }

  pub fn reset_user_mapping_rules(&mut self) {
    self.user_mapping_rules = None;
  }

  pub fn set_zone_id(&mut self, zone_id: i32) {
    self.zone_id = Some(zone_id);
  }

  pub fn with_zone_id(mut self, zone_id: i32) -> ZoneExtendedExtended {
    self.zone_id = Some(zone_id);
    self
  }

  pub fn zone_id(&self) -> Option<&i32> {
    self.zone_id.as_ref()
  }

  pub fn reset_zone_id(&mut self) {
    self.zone_id = None;
  }

}


