

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AntivirusServerExtended {
  /// A description for the server.
  #[serde(rename = "description")]
  description: Option<String>,
  /// Whether the server is enabled.
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  /// The icap url for the server.  This should have a format of: icap://host.domain:port/path
  #[serde(rename = "url")]
  url: Option<String>,
  #[serde(rename = "definitions")]
  definitions: Option<String>,
  /// A unique identifier for the server.
  #[serde(rename = "id")]
  id: Option<String>,
  /// The status of the server.
  #[serde(rename = "status")]
  status: Option<String>
}

