#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbSettingsGlobal {
    ///
    #[serde(rename = "settings")]
    pub settings: Option<::models::SmbSettingsGlobalSettings>,
}
