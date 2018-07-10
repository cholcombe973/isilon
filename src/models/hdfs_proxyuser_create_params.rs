/// HdfsProxyuserCreateParams : This is hdfs impersonation information of a proxyuser

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HdfsProxyuserCreateParams {
    /// The ID of the role.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Users or groups impersonated by proxyuser.
    #[serde(rename = "members")]
    pub members: Option<Vec<::models::AuthAccessAccessItemFileGroup>>,
    /// The name of the proxyuser.
    #[serde(rename = "name")]
    pub name: String,
}
