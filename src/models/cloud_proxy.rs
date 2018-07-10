/// CloudProxy : The configuration settings for a network proxy

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudProxy {
    /// A host name or network address for connecting to this proxy
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// A unique friendly name for this proxy configuration
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The password to connect to this proxy if required (write-only)
    #[serde(rename = "password")]
    pub password: Option<String>,
    /// The port used to connect to this proxy
    #[serde(rename = "port")]
    pub port: Option<i32>,
    /// The type of connection used to connect to this proxy
    #[serde(rename = "type")]
    pub _type: Option<String>,
    /// The username to connect to this proxy if required
    #[serde(rename = "username")]
    pub username: Option<String>,
}
