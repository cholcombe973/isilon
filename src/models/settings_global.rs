#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsGlobal {
    /// Specifies the properties for global authentication settings.
    #[serde(rename = "global_settings")]
    pub global_settings: Option<::models::SettingsGlobalGlobalSettings>,
}
