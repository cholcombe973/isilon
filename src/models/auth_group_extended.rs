#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGroupExtended {
    #[serde(rename = "dn")]
    pub dn: Option<String>,
    #[serde(rename = "dns_domain")]
    pub dns_domain: Option<String>,
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    /// If true, the GID was generated.
    #[serde(rename = "generated_gid")]
    pub generated_gid: Option<bool>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "gid")]
    pub gid: Option <crate::models::AuthAccessAccessItemFileGroup>,
    /// Specifies the user or group ID.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "member_of")]
    pub member_of: Option<Vec <crate::models::AuthAccessAccessItemFileGroup>>,
    /// Specifies a user or group name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "object_history")]
    pub object_history: Option<Vec <crate::models::AuthGroupObjectHistoryItem>>,
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    #[serde(rename = "sam_account_name")]
    pub sam_account_name: Option<String>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "sid")]
    pub sid: Option <crate::models::AuthAccessAccessItemFileGroup>,
    /// Specifies the object type.
    #[serde(rename = "type")]
    pub _type: String,
}
