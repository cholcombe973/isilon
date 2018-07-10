

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbLogLevel {
  /// Valid SMB logging levels
  #[serde(rename = "level")]
  level: Option<String>
}

