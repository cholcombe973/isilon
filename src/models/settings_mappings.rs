#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsMappings {
    #[serde(rename = "mappings")]
    pub mappings: Option<Vec <crate::models::SettingsMappingExtendedExtended>>,
}
