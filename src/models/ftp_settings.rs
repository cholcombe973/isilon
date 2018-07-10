#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FtpSettings {
    /// FTP settings.
    #[serde(rename = "settings")]
    pub settings: Option<::models::FtpSettingsSettings>,
}
