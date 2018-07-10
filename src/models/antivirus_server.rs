/// AntivirusServer : An ICAP server that contains virus definitions.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AntivirusServer {
    /// A description for the server.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether the server is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The icap url for the server.  This should have a format of: icap://host.domain:port/path
    #[serde(rename = "url")]
    pub url: Option<String>,
}
