

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGroupExtended {
  #[serde(rename = "dn")]
  dn: Option<String>,
  #[serde(rename = "dns_domain")]
  dns_domain: Option<String>,
  #[serde(rename = "domain")]
  domain: Option<String>,
  /// If true, the GID was generated.
  #[serde(rename = "generated_gid")]
  generated_gid: Option<bool>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "gid")]
  gid: Option<::models::AuthAccessAccessItemFileGroup>,
  /// Specifies the user or group ID.
  #[serde(rename = "id")]
  id: String,
  #[serde(rename = "member_of")]
  member_of: Option<Vec<::models::AuthAccessAccessItemFileGroup>>,
  /// Specifies a user or group name.
  #[serde(rename = "name")]
  name: String,
  #[serde(rename = "object_history")]
  object_history: Option<Vec<::models::AuthGroupObjectHistoryItem>>,
  #[serde(rename = "provider")]
  provider: Option<String>,
  #[serde(rename = "sam_account_name")]
  sam_account_name: Option<String>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "sid")]
  sid: Option<::models::AuthAccessAccessItemFileGroup>,
  /// Specifies the object type.
  #[serde(rename = "type")]
  _type: String
}

