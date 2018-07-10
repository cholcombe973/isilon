#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudSettings {
    ///
    #[serde(rename = "settings")]
    pub settings: Option<::models::CloudSettingsSettings>,
}
