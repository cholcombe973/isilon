

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdsProviderDomainsDomain {
  #[serde(rename = "client_site")]
  client_site: Option<String>,
  #[serde(rename = "dc_address")]
  dc_address: Option<String>,
  #[serde(rename = "dc_name")]
  dc_name: Option<String>,
  #[serde(rename = "dc_site")]
  dc_site: Option<String>,
  #[serde(rename = "domain")]
  domain: Option<String>,
  #[serde(rename = "guid")]
  guid: Option<String>,
  /// Specifies a unique identifier for every domain returned.
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "netbios_name")]
  netbios_name: Option<String>,
  #[serde(rename = "sid")]
  sid: Option<String>,
  /// Specifies the status of the domain.
  #[serde(rename = "status")]
  status: Option<String>,
  /// Specifies the type of trust for this domain. Options include 'primary', 'unknown', 'external', and 'forest'.
  #[serde(rename = "trust_type")]
  trust_type: Option<String>
}

