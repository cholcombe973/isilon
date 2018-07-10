

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsSettingsZoneSettings {
  /// If true, sends owners and groups as UIDs and GIDs when look up fails or if the 'nfsv4_no_name' property is set to 1.
  #[serde(rename = "nfsv4_allow_numeric_ids")]
  nfsv4_allow_numeric_ids: Option<bool>,
  /// Specifies the domain or realm through which users and groups are associated.
  #[serde(rename = "nfsv4_domain")]
  nfsv4_domain: Option<String>,
  /// If true, sends owners and groups without a domain name.
  #[serde(rename = "nfsv4_no_domain")]
  nfsv4_no_domain: Option<bool>,
  /// If true, sends UIDs and GIDs without a domain name.
  #[serde(rename = "nfsv4_no_domain_uids")]
  nfsv4_no_domain_uids: Option<bool>,
  /// If true, sends owners and groups as UIDs and GIDs.
  #[serde(rename = "nfsv4_no_names")]
  nfsv4_no_names: Option<bool>,
  /// If true, replaces the owner or group domain with an NFS domain name.
  #[serde(rename = "nfsv4_replace_domain")]
  nfsv4_replace_domain: Option<bool>,
  /// Specifies the access zones in which these settings apply.
  #[serde(rename = "zone")]
  zone: Option<String>
}

