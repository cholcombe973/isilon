

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsMappingExtended {
  /// The FQDN of destination domain to map to.
  #[serde(rename = "mapping")]
  mapping: String
}

