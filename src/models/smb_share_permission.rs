#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbSharePermission {
    /// Specifies the file system rights that are allowed or denied.
    #[serde(rename = "permission")]
    pub permission: String,
    /// Determines whether the permission is allowed or denied.
    #[serde(rename = "permission_type")]
    pub permission_type: String,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "trustee")]
    pub trustee:crate::models::AuthAccessAccessItemFileGroup,
}
