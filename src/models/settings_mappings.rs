

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsMappings {
  #[serde(rename = "mappings")]
  mappings: Option<Vec<::models::SettingsMappingExtendedExtended>>
}

