#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsMapping {
    /// Specifies the properties for global authentication setting.
    #[serde(rename = "mapping_settings")]
    pub mapping_settings: Option <crate::models::SettingsMappingMappingSettings>,
}
