

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudProxyCreateParams {
  /// A host name or network address for connecting to this proxy
  #[serde(rename = "host")]
  host: String,
  /// A unique friendly name for this proxy configuration
  #[serde(rename = "name")]
  name: String,
  /// The password to connect to this proxy if required (write-only)
  #[serde(rename = "password")]
  password: Option<String>,
  /// The port used to connect to this proxy
  #[serde(rename = "port")]
  port: i32,
  /// The type of connection used to connect to this proxy
  #[serde(rename = "type")]
  _type: String,
  /// The username to connect to this proxy if required
  #[serde(rename = "username")]
  username: Option<String>
}

