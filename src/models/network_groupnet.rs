#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkGroupnet {
    /// A description of the groupnet.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// DNS caching is enabled or disabled.
    #[serde(rename = "dns_cache_enabled")]
    pub dns_cache_enabled: Option<bool>,
    /// List of DNS resolver options.
    #[serde(rename = "dns_options")]
    pub dns_options: Option<Vec<String>>,
    /// List of DNS search suffixes.
    #[serde(rename = "dns_search")]
    pub dns_search: Option<Vec<String>>,
    /// List of Domain Name Server IP addresses.
    #[serde(rename = "dns_servers")]
    pub dns_servers: Option<Vec<String>>,
    /// The name of the groupnet.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Enable or disable appending nodes DNS search  list to client DNS inquiries directed at SmartConnect service IP.
    #[serde(rename = "server_side_dns_search")]
    pub server_side_dns_search: Option<bool>,
}
