

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessPointCreateParams {
  /// Absolute file system path of access point.
  #[serde(rename = "path")]
  path: String
}

