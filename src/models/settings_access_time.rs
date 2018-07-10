#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsAccessTime {
    ///
    #[serde(rename = "settings")]
    pub settings: Option<::models::SettingsAccessTimeSettings>,
}
