#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AntivirusServerExtended {
    /// A description for the server.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether the server is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The icap url for the server.  This should have a format of: icap://host.domain:port/path
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(rename = "definitions")]
    pub definitions: Option<String>,
    /// A unique identifier for the server.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The status of the server.
    #[serde(rename = "status")]
    pub status: Option<String>,
}
