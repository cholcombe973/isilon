

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthLogLevel {
  /// 
  #[serde(rename = "level")]
  level: Option<::models::AuthLogLevelLevel>
}

