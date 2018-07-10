

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkGroupnetExtended {
  /// A description of the groupnet.
  #[serde(rename = "description")]
  description: Option<String>,
  /// DNS caching is enabled or disabled.
  #[serde(rename = "dns_cache_enabled")]
  dns_cache_enabled: Option<bool>,
  /// List of DNS resolver options.
  #[serde(rename = "dns_options")]
  dns_options: Option<Vec<String>>,
  /// List of DNS search suffixes.
  #[serde(rename = "dns_search")]
  dns_search: Option<Vec<String>>,
  /// List of Domain Name Server IP addresses.
  #[serde(rename = "dns_servers")]
  dns_servers: Option<Vec<String>>,
  /// The name of the groupnet.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Enable or disable appending nodes DNS search  list to client DNS inquiries directed at SmartConnect service IP.
  #[serde(rename = "server_side_dns_search")]
  server_side_dns_search: Option<bool>,
  /// Unique Interface ID.
  #[serde(rename = "id")]
  id: Option<String>,
  /// Name of the subnets in the groupnet.
  #[serde(rename = "subnets")]
  subnets: Option<Vec<String>>
}

