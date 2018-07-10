#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HdfsProxyuser {
    /// The ID of the role.
    #[serde(rename = "id")]
    pub id: String,
    /// Users or groups impersonated by proxyuser.
    #[serde(rename = "members")]
    pub members: Vec<::models::AuthAccessAccessItemFileGroup>,
    /// The name of the proxyuser.
    #[serde(rename = "name")]
    pub name: String,
}
