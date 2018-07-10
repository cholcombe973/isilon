

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsMappingExtendedExtended {
  /// The FQDN of the source domain to map.
  #[serde(rename = "domain")]
  domain: String,
  #[serde(rename = "id")]
  id: Option<String>,
  /// The FQDN of destination domain to map to.
  #[serde(rename = "mapping")]
  mapping: String,
  /// The authentication provider type.
  #[serde(rename = "type")]
  _type: String
}

