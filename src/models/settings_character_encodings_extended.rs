#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsCharacterEncodingsExtended {
    /// Current character encoding.
    #[serde(rename = "current-encoding")]
    pub current_encoding: Option<String>,
}
