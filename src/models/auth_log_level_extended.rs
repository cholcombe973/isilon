

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthLogLevelExtended {
  /// Valid auth logging levels
  #[serde(rename = "level")]
  level: Option<String>
}

