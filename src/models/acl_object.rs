
#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AclObject {
  #[serde(rename = "accessrights")]
  accessrights: Option<Vec<String>>,
  #[serde(rename = "accesstype")]
  accesstype: Option<String>,
  #[serde(rename = "inherit_flags")]
  inherit_flags: Option<Vec<bool>>,
  #[serde(rename = "op")]
  op: Option<String>,
  #[serde(rename = "trustee")]
  trustee: Option<::models::MemberObject>,
}

