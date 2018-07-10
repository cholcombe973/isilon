

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WormDomainExtended {
  /// Specifies the autocommit time period for the domain in seconds.  After a file is in the domain without being modified for the specified time period, the file is automatically committed. If this parameter is set to null, there is no autocommit time, and files must be committed manually.
  #[serde(rename = "autocommit_offset")]
  autocommit_offset: Option<i32>,
  #[serde(rename = "default_retention")]
  default_retention: Option<String>,
  #[serde(rename = "max_retention")]
  max_retention: Option<String>,
  #[serde(rename = "min_retention")]
  min_retention: Option<String>,
  /// Specifies the override retention date for the domain. If this date is later than the retention date for any committed file, the file will remain protected until the override retention date.
  #[serde(rename = "override_date")]
  override_date: Option<i32>,
  /// When this value is set to 'on', files in this domain can be deleted through the privileged delete feature. If this value is set to 'disabled', privileged file deletes are permanently disabled and cannot be turned on again.
  #[serde(rename = "privileged_delete")]
  privileged_delete: String,
  /// Specifies whether the domain is an enterprise domain or a compliance domain. Compliance domains can not be created on enterprise clusters. Enterprise and compliance domains can be created on compliance clusters.
  #[serde(rename = "type")]
  _type: String,
  /// Specifies the system-assigned ID for the protection domain.
  #[serde(rename = "id")]
  id: i32,
  /// True if OneFS is still in the process of creating this domain and is unable to prevent files from being modified or deleted. If false, indicates that the domain is fully created and is able to prevent files from being modified or deleted.
  #[serde(rename = "incomplete")]
  incomplete: bool,
  /// Specifies the logical inode number (LIN) for the root of this domain.
  #[serde(rename = "lin")]
  lin: i32,
  /// Specifies the maximum amount of time, in seconds, that a file in this domain will be protected. This setting will override the retention period of any file committed with a longer retention period. If this parameter is set to null, an infinite length retention period is set.
  #[serde(rename = "max_modifies")]
  max_modifies: i32,
  /// Specifies the root path of this domain. Files in this directory and all sub-directories will be protected.
  #[serde(rename = "path")]
  path: String,
  /// Specifies the number of times this domain has been modified and the number of times the attributes for the domain have changed. A SmartLock domain can be modified a fixed number of times as defined by the 'max_modifies' parameter.
  #[serde(rename = "total_modifies")]
  total_modifies: i32
}

