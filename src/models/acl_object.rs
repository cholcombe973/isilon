#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AclObject {
    #[serde(rename = "accessrights")]
    pub accessrights: Option<Vec<String>>,
    #[serde(rename = "accesstype")]
    pub accesstype: Option<String>,
    #[serde(rename = "inherit_flags")]
    pub inherit_flags: Option<Vec<bool>>,
    #[serde(rename = "op")]
    pub op: Option<String>,
    #[serde(rename = "trustee")]
    pub trustee: Option <crate::models::MemberObject>,
}
