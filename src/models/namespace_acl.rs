#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceAcl {
    /// Provides the JSON array of access rights.
    #[serde(rename = "acl")]
    pub acl: Option<Vec <crate::models::AclObject>>,
    /// If the directory has access rights set, then this field is returned as acl. If the directory has POSIX permissions set, then this field is returned as mode.
    #[serde(rename = "authoritative")]
    pub authoritative: Option<String>,
    /// Provides the JSON object for the group persona of the owner.
    #[serde(rename = "group")]
    pub group: Option <crate::models::MemberObject>,
    /// Provides the POSIX mode.
    #[serde(rename = "mode")]
    pub mode: Option<String>,
    /// Provides the JSON object for the owner persona.
    #[serde(rename = "owner")]
    pub owner: Option <crate::models::MemberObject>,
}
