#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersSummaryProviderInstanceConnection {
    /// Specifies the IP address of the provider.
    #[serde(rename = "address")]
    pub address: Option<String>,
    /// Specifies the last time the server was contacted.
    #[serde(rename = "last_used")]
    pub last_used: Option<String>,
    /// Specifies the fully qualified domain name of the server.
    #[serde(rename = "server")]
    pub server: Option<String>,
}
