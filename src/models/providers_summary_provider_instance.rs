#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersSummaryProviderInstance {
    #[serde(rename = "active_server")]
    pub active_server: Option<String>,
    #[serde(rename = "client_site")]
    pub client_site: Option<String>,
    #[serde(rename = "connections")]
    pub connections: Option<Vec<::models::ProvidersSummaryProviderInstanceConnection>>,
    #[serde(rename = "dc_site")]
    pub dc_site: Option<String>,
    #[serde(rename = "forest")]
    pub forest: Option<String>,
    #[serde(rename = "groupnet")]
    pub groupnet: Option<String>,
    /// Specifies the ID of the provider.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Specifies the name of the provider.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Indicates the status of the provider.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Specifies the type of provider.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
