#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FsaSettings {
    ///
    #[serde(rename = "settings")]
    pub settings: Option<crate::models::FsaSettingsSettings>,
}
