

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthLogLevelLevel {
  /// Valid auth logging levels
  #[serde(rename = "lsass-level")]
  lsass_level: Option<String>,
  /// Valid auth logging levels
  #[serde(rename = "netlogon-level")]
  netlogon_level: Option<String>
}

