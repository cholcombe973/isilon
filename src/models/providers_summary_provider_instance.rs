

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersSummaryProviderInstance {
  #[serde(rename = "active_server")]
  active_server: Option<String>,
  #[serde(rename = "client_site")]
  client_site: Option<String>,
  #[serde(rename = "connections")]
  connections: Option<Vec<::models::ProvidersSummaryProviderInstanceConnection>>,
  #[serde(rename = "dc_site")]
  dc_site: Option<String>,
  #[serde(rename = "forest")]
  forest: Option<String>,
  #[serde(rename = "groupnet")]
  groupnet: Option<String>,
  /// Specifies the ID of the provider.
  #[serde(rename = "id")]
  id: Option<String>,
  /// Specifies the name of the provider.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Indicates the status of the provider.
  #[serde(rename = "status")]
  status: Option<String>,
  /// Specifies the type of provider.
  #[serde(rename = "type")]
  _type: Option<String>
}

