#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudProxyCreateParams {
    /// A host name or network address for connecting to this proxy
    #[serde(rename = "host")]
    pub host: String,
    /// A unique friendly name for this proxy configuration
    #[serde(rename = "name")]
    pub name: String,
    /// The password to connect to this proxy if required (write-only)
    #[serde(rename = "password")]
    pub password: Option<String>,
    /// The port used to connect to this proxy
    #[serde(rename = "port")]
    pub port: i32,
    /// The type of connection used to connect to this proxy
    #[serde(rename = "type")]
    pub _type: String,
    /// The username to connect to this proxy if required
    #[serde(rename = "username")]
    pub username: Option<String>,
}
