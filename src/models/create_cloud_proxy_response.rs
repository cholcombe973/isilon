#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCloudProxyResponse {
    /// The globally unique ID (guid) of the new proxy
    #[serde(rename = "id")]
    pub id: Option<String>,
}
