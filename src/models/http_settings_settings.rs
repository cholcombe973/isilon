#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpSettingsSettings {
    /// Enable Access Control Authentication
    #[serde(rename = "access_control")]
    pub access_control: Option<bool>,
    /// Enable Basic Authentication
    #[serde(rename = "basic_authentication")]
    pub basic_authentication: Option<bool>,
    /// Enable DAV specification
    #[serde(rename = "dav")]
    pub dav: Option<bool>,
    /// Enable HTTP access logging
    #[serde(rename = "enable_access_log")]
    pub enable_access_log: Option<bool>,
    /// Use HTTPS transport
    #[serde(rename = "https")]
    pub https: Option<bool>,
    /// Enable Integrated Authentication
    #[serde(rename = "integrated_authentication")]
    pub integrated_authentication: Option<bool>,
    /// Document root directory. Must be within /ifs.
    #[serde(rename = "server_root")]
    pub server_root: Option<String>,
    /// Enable/disable the HTTP service or redirect to WebUI.
    #[serde(rename = "service")]
    pub service: Option<String>,
}
