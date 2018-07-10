
/// CloudProxy : The configuration settings for a network proxy

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudProxy {
  /// A host name or network address for connecting to this proxy
  #[serde(rename = "host")]
  host: Option<String>,
  /// A unique friendly name for this proxy configuration
  #[serde(rename = "name")]
  name: Option<String>,
  /// The password to connect to this proxy if required (write-only)
  #[serde(rename = "password")]
  password: Option<String>,
  /// The port used to connect to this proxy
  #[serde(rename = "port")]
  port: Option<i32>,
  /// The type of connection used to connect to this proxy
  #[serde(rename = "type")]
  _type: Option<String>,
  /// The username to connect to this proxy if required
  #[serde(rename = "username")]
  username: Option<String>
}

