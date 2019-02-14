#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAccessAccessItem {
    /// Specifies properties for access rights.
    #[serde(rename = "file")]
    pub file: Option <crate::models::AuthAccessAccessItemFile>,
    /// Specifies the ID of the user.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Specifies the permissions that the user has on the share.
    #[serde(rename = "share")]
    pub share: Option <crate::models::AuthAccessAccessItemShare>,
    /// Specifies the persona for the user.
    #[serde(rename = "user")]
    pub user: Option <crate::models::AuthAccessAccessItemShareEffectiveUser>,
}
