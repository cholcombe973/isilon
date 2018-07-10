

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceAcl {
  /// Provides the JSON array of access rights.
  #[serde(rename = "acl")]
  acl: Option<Vec<::models::AclObject>>,
  /// If the directory has access rights set, then this field is returned as acl. If the directory has POSIX permissions set, then this field is returned as mode.
  #[serde(rename = "authoritative")]
  authoritative: Option<String>,
  /// Provides the JSON object for the group persona of the owner.
  #[serde(rename = "group")]
  group: Option<::models::MemberObject>,
  /// Provides the POSIX mode.
  #[serde(rename = "mode")]
  mode: Option<String>,
  /// Provides the JSON object for the owner persona.
  #[serde(rename = "owner")]
  owner: Option<::models::MemberObject>
}

