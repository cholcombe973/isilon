#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsMappingExtendedExtended {
    /// The FQDN of the source domain to map.
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The FQDN of destination domain to map to.
    #[serde(rename = "mapping")]
    pub mapping: String,
    /// The authentication provider type.
    #[serde(rename = "type")]
    pub _type: String,
}
