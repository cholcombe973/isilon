

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsLogLevel {
  /// Valid NFS logging levels
  #[serde(rename = "level")]
  level: Option<String>
}

