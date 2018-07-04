/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ProvidersAdsItem : Specifies the properties for an ADS authentication provider.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersAdsItem {
  /// Specifies the machine account name when creating a SAM account with Active Directory. The default cluster name is called 'default'.
  #[serde(rename = "account")]
  account: Option<String>,
  /// Allocates an ID for an unmapped Active Directory (ADS) group. ADS groups without GIDs can be proactively assigned a GID by the ID mapper. If the ID mapper option is disabled, GIDs are not proactively assigned, and when a primary group for a user does not include a GID, the system may allocate one. 
  #[serde(rename = "allocate_gids")]
  allocate_gids: Option<bool>,
  /// Allocates a user ID for an unmapped Active Directory (ADS) user. ADS users without UIDs can be proactively assigned a UID by the ID mapper. IF the ID mapper option is disabled, UIDs are not proactively assigned, and when an identify for a user does not include a UID, the system may allocate one.
  #[serde(rename = "allocate_uids")]
  allocate_uids: Option<bool>,
  /// Enables lookup of unqualified user names in the primary domain.
  #[serde(rename = "assume_default_domain")]
  assume_default_domain: Option<bool>,
  /// Enables authentication and identity management through the authentication provider.
  #[serde(rename = "authentication")]
  authentication: Option<bool>,
  /// Specifies the time in seconds between provider online checks.
  #[serde(rename = "check_online_interval")]
  check_online_interval: Option<i32>,
  /// Specifies the current time for the domain controllers.
  #[serde(rename = "controller_time")]
  controller_time: Option<i32>,
  /// Automatically creates a home directory on the first login.
  #[serde(rename = "create_home_directory")]
  create_home_directory: Option<bool>,
  /// Specifies the DNS search domain. Set this parameter if the DNS search domain has a unique name or address.
  #[serde(rename = "dns_domain")]
  dns_domain: Option<String>,
  /// Sends an alert if the domain goes offline.
  #[serde(rename = "domain_offline_alerts")]
  domain_offline_alerts: Option<bool>,
  /// Sets list of groups that can be resolved.
  #[serde(rename = "findable_groups")]
  findable_groups: Option<Vec<String>>,
  /// Sets list of users that can be resolved.
  #[serde(rename = "findable_users")]
  findable_users: Option<Vec<String>>,
  /// Groupnet identifier.
  #[serde(rename = "groupnet")]
  groupnet: Option<String>,
  /// Specifies the path to the home directory template.
  #[serde(rename = "home_directory_template")]
  home_directory_template: Option<String>,
  /// If set to true, ignores all trusted domains.
  #[serde(rename = "ignore_all_trusts")]
  ignore_all_trusts: Option<bool>,
  /// Includes trusted domains when 'ignore_all_trusts' is set to false.
  #[serde(rename = "ignored_trusted_domains")]
  ignored_trusted_domains: Option<Vec<String>>,
  /// Includes trusted domains when 'ignore_all_trusts' is set to true.
  #[serde(rename = "include_trusted_domains")]
  include_trusted_domains: Option<Vec<String>>,
  /// Specifies Active Directory provider instance.
  #[serde(rename = "instance")]
  instance: Option<String>,
  /// Determines if connecting through HDFS with Kerberos.
  #[serde(rename = "kerberos_hdfs_spn")]
  kerberos_hdfs_spn: Option<bool>,
  /// Determines if connecting through NFS with Kerberos.
  #[serde(rename = "kerberos_nfs_spn")]
  kerberos_nfs_spn: Option<bool>,
  /// Enables encryption and signing on LDAP requests.
  #[serde(rename = "ldap_sign_and_seal")]
  ldap_sign_and_seal: Option<bool>,
  /// Specifies the login shell path.
  #[serde(rename = "login_shell")]
  login_shell: Option<String>,
  /// Limits user and group lookups to the specified domains.
  #[serde(rename = "lookup_domains")]
  lookup_domains: Option<Vec<String>>,
  /// Looks up AD groups in other providers before allocating a group ID.
  #[serde(rename = "lookup_groups")]
  lookup_groups: Option<bool>,
  /// Normalizes AD group names to lowercase before look up.
  #[serde(rename = "lookup_normalize_groups")]
  lookup_normalize_groups: Option<bool>,
  /// Normalize AD user names to lowercase before look up.
  #[serde(rename = "lookup_normalize_users")]
  lookup_normalize_users: Option<bool>,
  /// Looks up AD users in other providers before allocating a user ID.
  #[serde(rename = "lookup_users")]
  lookup_users: Option<bool>,
  /// Specifies name to join AD as.
  #[serde(rename = "machine_name")]
  machine_name: Option<String>,
  /// Enables periodic changes of the machine password for security.
  #[serde(rename = "machine_password_changes")]
  machine_password_changes: Option<bool>,
  /// Sets maximum age of a password in seconds.
  #[serde(rename = "machine_password_lifespan")]
  machine_password_lifespan: Option<i32>,
  /// Specifies the Active Directory provider name.
  #[serde(rename = "name")]
  name: String,
  /// Specifies the domain controller for which the node has affinity.
  #[serde(rename = "node_dc_affinity")]
  node_dc_affinity: Option<String>,
  /// Specifies the timeout for the domain controller for which the local node has affinity.
  #[serde(rename = "node_dc_affinity_timeout")]
  node_dc_affinity_timeout: Option<i32>,
  /// Enables the Active Directory provider to respond to 'getpwent' and 'getgrent' requests.
  #[serde(rename = "nss_enumeration")]
  nss_enumeration: Option<bool>,
  /// Specifies the organizational unit.
  #[serde(rename = "organizational_unit")]
  organizational_unit: Option<String>,
  /// Specifies the password used during domain join.
  #[serde(rename = "password")]
  password: String,
  /// Check the provider for filtered lists of findable and unfindable users and groups.
  #[serde(rename = "restrict_findable")]
  restrict_findable: Option<bool>,
  /// Specifies whether to support RFC 2307 attributes on ADS domain controllers.
  #[serde(rename = "sfu_support")]
  sfu_support: Option<String>,
  /// Stores SFU mappings permanently in the ID mapper.
  #[serde(rename = "store_sfu_mappings")]
  store_sfu_mappings: Option<bool>,
  /// Specifies groups that cannot be resolved by the provider.
  #[serde(rename = "unfindable_groups")]
  unfindable_groups: Option<Vec<String>>,
  /// Specifies users that cannot be resolved by the provider.
  #[serde(rename = "unfindable_users")]
  unfindable_users: Option<Vec<String>>,
  /// Specifies the user name that has permission to join a machine to the given domain.
  #[serde(rename = "user")]
  user: String
}

impl ProvidersAdsItem {
  /// Specifies the properties for an ADS authentication provider.
  pub fn new(name: String, password: String, user: String) -> ProvidersAdsItem {
    ProvidersAdsItem {
      account: None,
      allocate_gids: None,
      allocate_uids: None,
      assume_default_domain: None,
      authentication: None,
      check_online_interval: None,
      controller_time: None,
      create_home_directory: None,
      dns_domain: None,
      domain_offline_alerts: None,
      findable_groups: None,
      findable_users: None,
      groupnet: None,
      home_directory_template: None,
      ignore_all_trusts: None,
      ignored_trusted_domains: None,
      include_trusted_domains: None,
      instance: None,
      kerberos_hdfs_spn: None,
      kerberos_nfs_spn: None,
      ldap_sign_and_seal: None,
      login_shell: None,
      lookup_domains: None,
      lookup_groups: None,
      lookup_normalize_groups: None,
      lookup_normalize_users: None,
      lookup_users: None,
      machine_name: None,
      machine_password_changes: None,
      machine_password_lifespan: None,
      name: name,
      node_dc_affinity: None,
      node_dc_affinity_timeout: None,
      nss_enumeration: None,
      organizational_unit: None,
      password: password,
      restrict_findable: None,
      sfu_support: None,
      store_sfu_mappings: None,
      unfindable_groups: None,
      unfindable_users: None,
      user: user
    }
  }

  pub fn set_account(&mut self, account: String) {
    self.account = Some(account);
  }

  pub fn with_account(mut self, account: String) -> ProvidersAdsItem {
    self.account = Some(account);
    self
  }

  pub fn account(&self) -> Option<&String> {
    self.account.as_ref()
  }

  pub fn reset_account(&mut self) {
    self.account = None;
  }

  pub fn set_allocate_gids(&mut self, allocate_gids: bool) {
    self.allocate_gids = Some(allocate_gids);
  }

  pub fn with_allocate_gids(mut self, allocate_gids: bool) -> ProvidersAdsItem {
    self.allocate_gids = Some(allocate_gids);
    self
  }

  pub fn allocate_gids(&self) -> Option<&bool> {
    self.allocate_gids.as_ref()
  }

  pub fn reset_allocate_gids(&mut self) {
    self.allocate_gids = None;
  }

  pub fn set_allocate_uids(&mut self, allocate_uids: bool) {
    self.allocate_uids = Some(allocate_uids);
  }

  pub fn with_allocate_uids(mut self, allocate_uids: bool) -> ProvidersAdsItem {
    self.allocate_uids = Some(allocate_uids);
    self
  }

  pub fn allocate_uids(&self) -> Option<&bool> {
    self.allocate_uids.as_ref()
  }

  pub fn reset_allocate_uids(&mut self) {
    self.allocate_uids = None;
  }

  pub fn set_assume_default_domain(&mut self, assume_default_domain: bool) {
    self.assume_default_domain = Some(assume_default_domain);
  }

  pub fn with_assume_default_domain(mut self, assume_default_domain: bool) -> ProvidersAdsItem {
    self.assume_default_domain = Some(assume_default_domain);
    self
  }

  pub fn assume_default_domain(&self) -> Option<&bool> {
    self.assume_default_domain.as_ref()
  }

  pub fn reset_assume_default_domain(&mut self) {
    self.assume_default_domain = None;
  }

  pub fn set_authentication(&mut self, authentication: bool) {
    self.authentication = Some(authentication);
  }

  pub fn with_authentication(mut self, authentication: bool) -> ProvidersAdsItem {
    self.authentication = Some(authentication);
    self
  }

  pub fn authentication(&self) -> Option<&bool> {
    self.authentication.as_ref()
  }

  pub fn reset_authentication(&mut self) {
    self.authentication = None;
  }

  pub fn set_check_online_interval(&mut self, check_online_interval: i32) {
    self.check_online_interval = Some(check_online_interval);
  }

  pub fn with_check_online_interval(mut self, check_online_interval: i32) -> ProvidersAdsItem {
    self.check_online_interval = Some(check_online_interval);
    self
  }

  pub fn check_online_interval(&self) -> Option<&i32> {
    self.check_online_interval.as_ref()
  }

  pub fn reset_check_online_interval(&mut self) {
    self.check_online_interval = None;
  }

  pub fn set_controller_time(&mut self, controller_time: i32) {
    self.controller_time = Some(controller_time);
  }

  pub fn with_controller_time(mut self, controller_time: i32) -> ProvidersAdsItem {
    self.controller_time = Some(controller_time);
    self
  }

  pub fn controller_time(&self) -> Option<&i32> {
    self.controller_time.as_ref()
  }

  pub fn reset_controller_time(&mut self) {
    self.controller_time = None;
  }

  pub fn set_create_home_directory(&mut self, create_home_directory: bool) {
    self.create_home_directory = Some(create_home_directory);
  }

  pub fn with_create_home_directory(mut self, create_home_directory: bool) -> ProvidersAdsItem {
    self.create_home_directory = Some(create_home_directory);
    self
  }

  pub fn create_home_directory(&self) -> Option<&bool> {
    self.create_home_directory.as_ref()
  }

  pub fn reset_create_home_directory(&mut self) {
    self.create_home_directory = None;
  }

  pub fn set_dns_domain(&mut self, dns_domain: String) {
    self.dns_domain = Some(dns_domain);
  }

  pub fn with_dns_domain(mut self, dns_domain: String) -> ProvidersAdsItem {
    self.dns_domain = Some(dns_domain);
    self
  }

  pub fn dns_domain(&self) -> Option<&String> {
    self.dns_domain.as_ref()
  }

  pub fn reset_dns_domain(&mut self) {
    self.dns_domain = None;
  }

  pub fn set_domain_offline_alerts(&mut self, domain_offline_alerts: bool) {
    self.domain_offline_alerts = Some(domain_offline_alerts);
  }

  pub fn with_domain_offline_alerts(mut self, domain_offline_alerts: bool) -> ProvidersAdsItem {
    self.domain_offline_alerts = Some(domain_offline_alerts);
    self
  }

  pub fn domain_offline_alerts(&self) -> Option<&bool> {
    self.domain_offline_alerts.as_ref()
  }

  pub fn reset_domain_offline_alerts(&mut self) {
    self.domain_offline_alerts = None;
  }

  pub fn set_findable_groups(&mut self, findable_groups: Vec<String>) {
    self.findable_groups = Some(findable_groups);
  }

  pub fn with_findable_groups(mut self, findable_groups: Vec<String>) -> ProvidersAdsItem {
    self.findable_groups = Some(findable_groups);
    self
  }

  pub fn findable_groups(&self) -> Option<&Vec<String>> {
    self.findable_groups.as_ref()
  }

  pub fn reset_findable_groups(&mut self) {
    self.findable_groups = None;
  }

  pub fn set_findable_users(&mut self, findable_users: Vec<String>) {
    self.findable_users = Some(findable_users);
  }

  pub fn with_findable_users(mut self, findable_users: Vec<String>) -> ProvidersAdsItem {
    self.findable_users = Some(findable_users);
    self
  }

  pub fn findable_users(&self) -> Option<&Vec<String>> {
    self.findable_users.as_ref()
  }

  pub fn reset_findable_users(&mut self) {
    self.findable_users = None;
  }

  pub fn set_groupnet(&mut self, groupnet: String) {
    self.groupnet = Some(groupnet);
  }

  pub fn with_groupnet(mut self, groupnet: String) -> ProvidersAdsItem {
    self.groupnet = Some(groupnet);
    self
  }

  pub fn groupnet(&self) -> Option<&String> {
    self.groupnet.as_ref()
  }

  pub fn reset_groupnet(&mut self) {
    self.groupnet = None;
  }

  pub fn set_home_directory_template(&mut self, home_directory_template: String) {
    self.home_directory_template = Some(home_directory_template);
  }

  pub fn with_home_directory_template(mut self, home_directory_template: String) -> ProvidersAdsItem {
    self.home_directory_template = Some(home_directory_template);
    self
  }

  pub fn home_directory_template(&self) -> Option<&String> {
    self.home_directory_template.as_ref()
  }

  pub fn reset_home_directory_template(&mut self) {
    self.home_directory_template = None;
  }

  pub fn set_ignore_all_trusts(&mut self, ignore_all_trusts: bool) {
    self.ignore_all_trusts = Some(ignore_all_trusts);
  }

  pub fn with_ignore_all_trusts(mut self, ignore_all_trusts: bool) -> ProvidersAdsItem {
    self.ignore_all_trusts = Some(ignore_all_trusts);
    self
  }

  pub fn ignore_all_trusts(&self) -> Option<&bool> {
    self.ignore_all_trusts.as_ref()
  }

  pub fn reset_ignore_all_trusts(&mut self) {
    self.ignore_all_trusts = None;
  }

  pub fn set_ignored_trusted_domains(&mut self, ignored_trusted_domains: Vec<String>) {
    self.ignored_trusted_domains = Some(ignored_trusted_domains);
  }

  pub fn with_ignored_trusted_domains(mut self, ignored_trusted_domains: Vec<String>) -> ProvidersAdsItem {
    self.ignored_trusted_domains = Some(ignored_trusted_domains);
    self
  }

  pub fn ignored_trusted_domains(&self) -> Option<&Vec<String>> {
    self.ignored_trusted_domains.as_ref()
  }

  pub fn reset_ignored_trusted_domains(&mut self) {
    self.ignored_trusted_domains = None;
  }

  pub fn set_include_trusted_domains(&mut self, include_trusted_domains: Vec<String>) {
    self.include_trusted_domains = Some(include_trusted_domains);
  }

  pub fn with_include_trusted_domains(mut self, include_trusted_domains: Vec<String>) -> ProvidersAdsItem {
    self.include_trusted_domains = Some(include_trusted_domains);
    self
  }

  pub fn include_trusted_domains(&self) -> Option<&Vec<String>> {
    self.include_trusted_domains.as_ref()
  }

  pub fn reset_include_trusted_domains(&mut self) {
    self.include_trusted_domains = None;
  }

  pub fn set_instance(&mut self, instance: String) {
    self.instance = Some(instance);
  }

  pub fn with_instance(mut self, instance: String) -> ProvidersAdsItem {
    self.instance = Some(instance);
    self
  }

  pub fn instance(&self) -> Option<&String> {
    self.instance.as_ref()
  }

  pub fn reset_instance(&mut self) {
    self.instance = None;
  }

  pub fn set_kerberos_hdfs_spn(&mut self, kerberos_hdfs_spn: bool) {
    self.kerberos_hdfs_spn = Some(kerberos_hdfs_spn);
  }

  pub fn with_kerberos_hdfs_spn(mut self, kerberos_hdfs_spn: bool) -> ProvidersAdsItem {
    self.kerberos_hdfs_spn = Some(kerberos_hdfs_spn);
    self
  }

  pub fn kerberos_hdfs_spn(&self) -> Option<&bool> {
    self.kerberos_hdfs_spn.as_ref()
  }

  pub fn reset_kerberos_hdfs_spn(&mut self) {
    self.kerberos_hdfs_spn = None;
  }

  pub fn set_kerberos_nfs_spn(&mut self, kerberos_nfs_spn: bool) {
    self.kerberos_nfs_spn = Some(kerberos_nfs_spn);
  }

  pub fn with_kerberos_nfs_spn(mut self, kerberos_nfs_spn: bool) -> ProvidersAdsItem {
    self.kerberos_nfs_spn = Some(kerberos_nfs_spn);
    self
  }

  pub fn kerberos_nfs_spn(&self) -> Option<&bool> {
    self.kerberos_nfs_spn.as_ref()
  }

  pub fn reset_kerberos_nfs_spn(&mut self) {
    self.kerberos_nfs_spn = None;
  }

  pub fn set_ldap_sign_and_seal(&mut self, ldap_sign_and_seal: bool) {
    self.ldap_sign_and_seal = Some(ldap_sign_and_seal);
  }

  pub fn with_ldap_sign_and_seal(mut self, ldap_sign_and_seal: bool) -> ProvidersAdsItem {
    self.ldap_sign_and_seal = Some(ldap_sign_and_seal);
    self
  }

  pub fn ldap_sign_and_seal(&self) -> Option<&bool> {
    self.ldap_sign_and_seal.as_ref()
  }

  pub fn reset_ldap_sign_and_seal(&mut self) {
    self.ldap_sign_and_seal = None;
  }

  pub fn set_login_shell(&mut self, login_shell: String) {
    self.login_shell = Some(login_shell);
  }

  pub fn with_login_shell(mut self, login_shell: String) -> ProvidersAdsItem {
    self.login_shell = Some(login_shell);
    self
  }

  pub fn login_shell(&self) -> Option<&String> {
    self.login_shell.as_ref()
  }

  pub fn reset_login_shell(&mut self) {
    self.login_shell = None;
  }

  pub fn set_lookup_domains(&mut self, lookup_domains: Vec<String>) {
    self.lookup_domains = Some(lookup_domains);
  }

  pub fn with_lookup_domains(mut self, lookup_domains: Vec<String>) -> ProvidersAdsItem {
    self.lookup_domains = Some(lookup_domains);
    self
  }

  pub fn lookup_domains(&self) -> Option<&Vec<String>> {
    self.lookup_domains.as_ref()
  }

  pub fn reset_lookup_domains(&mut self) {
    self.lookup_domains = None;
  }

  pub fn set_lookup_groups(&mut self, lookup_groups: bool) {
    self.lookup_groups = Some(lookup_groups);
  }

  pub fn with_lookup_groups(mut self, lookup_groups: bool) -> ProvidersAdsItem {
    self.lookup_groups = Some(lookup_groups);
    self
  }

  pub fn lookup_groups(&self) -> Option<&bool> {
    self.lookup_groups.as_ref()
  }

  pub fn reset_lookup_groups(&mut self) {
    self.lookup_groups = None;
  }

  pub fn set_lookup_normalize_groups(&mut self, lookup_normalize_groups: bool) {
    self.lookup_normalize_groups = Some(lookup_normalize_groups);
  }

  pub fn with_lookup_normalize_groups(mut self, lookup_normalize_groups: bool) -> ProvidersAdsItem {
    self.lookup_normalize_groups = Some(lookup_normalize_groups);
    self
  }

  pub fn lookup_normalize_groups(&self) -> Option<&bool> {
    self.lookup_normalize_groups.as_ref()
  }

  pub fn reset_lookup_normalize_groups(&mut self) {
    self.lookup_normalize_groups = None;
  }

  pub fn set_lookup_normalize_users(&mut self, lookup_normalize_users: bool) {
    self.lookup_normalize_users = Some(lookup_normalize_users);
  }

  pub fn with_lookup_normalize_users(mut self, lookup_normalize_users: bool) -> ProvidersAdsItem {
    self.lookup_normalize_users = Some(lookup_normalize_users);
    self
  }

  pub fn lookup_normalize_users(&self) -> Option<&bool> {
    self.lookup_normalize_users.as_ref()
  }

  pub fn reset_lookup_normalize_users(&mut self) {
    self.lookup_normalize_users = None;
  }

  pub fn set_lookup_users(&mut self, lookup_users: bool) {
    self.lookup_users = Some(lookup_users);
  }

  pub fn with_lookup_users(mut self, lookup_users: bool) -> ProvidersAdsItem {
    self.lookup_users = Some(lookup_users);
    self
  }

  pub fn lookup_users(&self) -> Option<&bool> {
    self.lookup_users.as_ref()
  }

  pub fn reset_lookup_users(&mut self) {
    self.lookup_users = None;
  }

  pub fn set_machine_name(&mut self, machine_name: String) {
    self.machine_name = Some(machine_name);
  }

  pub fn with_machine_name(mut self, machine_name: String) -> ProvidersAdsItem {
    self.machine_name = Some(machine_name);
    self
  }

  pub fn machine_name(&self) -> Option<&String> {
    self.machine_name.as_ref()
  }

  pub fn reset_machine_name(&mut self) {
    self.machine_name = None;
  }

  pub fn set_machine_password_changes(&mut self, machine_password_changes: bool) {
    self.machine_password_changes = Some(machine_password_changes);
  }

  pub fn with_machine_password_changes(mut self, machine_password_changes: bool) -> ProvidersAdsItem {
    self.machine_password_changes = Some(machine_password_changes);
    self
  }

  pub fn machine_password_changes(&self) -> Option<&bool> {
    self.machine_password_changes.as_ref()
  }

  pub fn reset_machine_password_changes(&mut self) {
    self.machine_password_changes = None;
  }

  pub fn set_machine_password_lifespan(&mut self, machine_password_lifespan: i32) {
    self.machine_password_lifespan = Some(machine_password_lifespan);
  }

  pub fn with_machine_password_lifespan(mut self, machine_password_lifespan: i32) -> ProvidersAdsItem {
    self.machine_password_lifespan = Some(machine_password_lifespan);
    self
  }

  pub fn machine_password_lifespan(&self) -> Option<&i32> {
    self.machine_password_lifespan.as_ref()
  }

  pub fn reset_machine_password_lifespan(&mut self) {
    self.machine_password_lifespan = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> ProvidersAdsItem {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_node_dc_affinity(&mut self, node_dc_affinity: String) {
    self.node_dc_affinity = Some(node_dc_affinity);
  }

  pub fn with_node_dc_affinity(mut self, node_dc_affinity: String) -> ProvidersAdsItem {
    self.node_dc_affinity = Some(node_dc_affinity);
    self
  }

  pub fn node_dc_affinity(&self) -> Option<&String> {
    self.node_dc_affinity.as_ref()
  }

  pub fn reset_node_dc_affinity(&mut self) {
    self.node_dc_affinity = None;
  }

  pub fn set_node_dc_affinity_timeout(&mut self, node_dc_affinity_timeout: i32) {
    self.node_dc_affinity_timeout = Some(node_dc_affinity_timeout);
  }

  pub fn with_node_dc_affinity_timeout(mut self, node_dc_affinity_timeout: i32) -> ProvidersAdsItem {
    self.node_dc_affinity_timeout = Some(node_dc_affinity_timeout);
    self
  }

  pub fn node_dc_affinity_timeout(&self) -> Option<&i32> {
    self.node_dc_affinity_timeout.as_ref()
  }

  pub fn reset_node_dc_affinity_timeout(&mut self) {
    self.node_dc_affinity_timeout = None;
  }

  pub fn set_nss_enumeration(&mut self, nss_enumeration: bool) {
    self.nss_enumeration = Some(nss_enumeration);
  }

  pub fn with_nss_enumeration(mut self, nss_enumeration: bool) -> ProvidersAdsItem {
    self.nss_enumeration = Some(nss_enumeration);
    self
  }

  pub fn nss_enumeration(&self) -> Option<&bool> {
    self.nss_enumeration.as_ref()
  }

  pub fn reset_nss_enumeration(&mut self) {
    self.nss_enumeration = None;
  }

  pub fn set_organizational_unit(&mut self, organizational_unit: String) {
    self.organizational_unit = Some(organizational_unit);
  }

  pub fn with_organizational_unit(mut self, organizational_unit: String) -> ProvidersAdsItem {
    self.organizational_unit = Some(organizational_unit);
    self
  }

  pub fn organizational_unit(&self) -> Option<&String> {
    self.organizational_unit.as_ref()
  }

  pub fn reset_organizational_unit(&mut self) {
    self.organizational_unit = None;
  }

  pub fn set_password(&mut self, password: String) {
    self.password = password;
  }

  pub fn with_password(mut self, password: String) -> ProvidersAdsItem {
    self.password = password;
    self
  }

  pub fn password(&self) -> &String {
    &self.password
  }


  pub fn set_restrict_findable(&mut self, restrict_findable: bool) {
    self.restrict_findable = Some(restrict_findable);
  }

  pub fn with_restrict_findable(mut self, restrict_findable: bool) -> ProvidersAdsItem {
    self.restrict_findable = Some(restrict_findable);
    self
  }

  pub fn restrict_findable(&self) -> Option<&bool> {
    self.restrict_findable.as_ref()
  }

  pub fn reset_restrict_findable(&mut self) {
    self.restrict_findable = None;
  }

  pub fn set_sfu_support(&mut self, sfu_support: String) {
    self.sfu_support = Some(sfu_support);
  }

  pub fn with_sfu_support(mut self, sfu_support: String) -> ProvidersAdsItem {
    self.sfu_support = Some(sfu_support);
    self
  }

  pub fn sfu_support(&self) -> Option<&String> {
    self.sfu_support.as_ref()
  }

  pub fn reset_sfu_support(&mut self) {
    self.sfu_support = None;
  }

  pub fn set_store_sfu_mappings(&mut self, store_sfu_mappings: bool) {
    self.store_sfu_mappings = Some(store_sfu_mappings);
  }

  pub fn with_store_sfu_mappings(mut self, store_sfu_mappings: bool) -> ProvidersAdsItem {
    self.store_sfu_mappings = Some(store_sfu_mappings);
    self
  }

  pub fn store_sfu_mappings(&self) -> Option<&bool> {
    self.store_sfu_mappings.as_ref()
  }

  pub fn reset_store_sfu_mappings(&mut self) {
    self.store_sfu_mappings = None;
  }

  pub fn set_unfindable_groups(&mut self, unfindable_groups: Vec<String>) {
    self.unfindable_groups = Some(unfindable_groups);
  }

  pub fn with_unfindable_groups(mut self, unfindable_groups: Vec<String>) -> ProvidersAdsItem {
    self.unfindable_groups = Some(unfindable_groups);
    self
  }

  pub fn unfindable_groups(&self) -> Option<&Vec<String>> {
    self.unfindable_groups.as_ref()
  }

  pub fn reset_unfindable_groups(&mut self) {
    self.unfindable_groups = None;
  }

  pub fn set_unfindable_users(&mut self, unfindable_users: Vec<String>) {
    self.unfindable_users = Some(unfindable_users);
  }

  pub fn with_unfindable_users(mut self, unfindable_users: Vec<String>) -> ProvidersAdsItem {
    self.unfindable_users = Some(unfindable_users);
    self
  }

  pub fn unfindable_users(&self) -> Option<&Vec<String>> {
    self.unfindable_users.as_ref()
  }

  pub fn reset_unfindable_users(&mut self) {
    self.unfindable_users = None;
  }

  pub fn set_user(&mut self, user: String) {
    self.user = user;
  }

  pub fn with_user(mut self, user: String) -> ProvidersAdsItem {
    self.user = user;
    self
  }

  pub fn user(&self) -> &String {
    &self.user
  }


}


