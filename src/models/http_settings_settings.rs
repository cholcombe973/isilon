

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpSettingsSettings {
  /// Enable Access Control Authentication
  #[serde(rename = "access_control")]
  access_control: Option<bool>,
  /// Enable Basic Authentication
  #[serde(rename = "basic_authentication")]
  basic_authentication: Option<bool>,
  /// Enable DAV specification
  #[serde(rename = "dav")]
  dav: Option<bool>,
  /// Enable HTTP access logging
  #[serde(rename = "enable_access_log")]
  enable_access_log: Option<bool>,
  /// Use HTTPS transport
  #[serde(rename = "https")]
  https: Option<bool>,
  /// Enable Integrated Authentication
  #[serde(rename = "integrated_authentication")]
  integrated_authentication: Option<bool>,
  /// Document root directory. Must be within /ifs.
  #[serde(rename = "server_root")]
  server_root: Option<String>,
  /// Enable/disable the HTTP service or redirect to WebUI.
  #[serde(rename = "service")]
  service: Option<String>
}

