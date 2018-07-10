
/// SettingsAclsExtended : ACL policies settings.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsAclsExtended {
  /// Access checks (chmod, chown).
  #[serde(rename = "access")]
  access: Option<String>,
  /// Displayed mode bits.
  #[serde(rename = "calcmode")]
  calcmode: Option<String>,
  /// Approximate group mode bits when ACL exists.
  #[serde(rename = "calcmode_group")]
  calcmode_group: Option<String>,
  /// Approximate owner mode bits when ACL exists.
  #[serde(rename = "calcmode_owner")]
  calcmode_owner: Option<String>,
  /// chmod on files with existing ACLs.
  #[serde(rename = "chmod")]
  chmod: Option<String>,
  /// chmod (007) on files with existing ACLs.
  #[serde(rename = "chmod_007")]
  chmod_007: Option<String>,
  /// ACLs created on directories by UNIX chmod.
  #[serde(rename = "chmod_inheritable")]
  chmod_inheritable: Option<String>,
  /// chown/chgrp on files with existing ACLs.
  #[serde(rename = "chown")]
  chown: Option<String>,
  /// ACL creation over SMB.
  #[serde(rename = "create_over_smb")]
  create_over_smb: Option<String>,
  ///  Read only DOS attribute.
  #[serde(rename = "dos_attr")]
  dos_attr: Option<String>,
  /// Group owner inheritance.
  #[serde(rename = "group_owner_inheritance")]
  group_owner_inheritance: Option<String>,
  /// Treatment of 'rwx' permissions.
  #[serde(rename = "rwx")]
  rwx: Option<String>,
  /// Synthetic 'deny' ACEs.
  #[serde(rename = "synthetic_denies")]
  synthetic_denies: Option<String>,
  /// Access check (utimes)
  #[serde(rename = "utimes")]
  utimes: Option<String>
}

