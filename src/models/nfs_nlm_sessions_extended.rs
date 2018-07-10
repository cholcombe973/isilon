

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsNlmSessionsExtended {
  #[serde(rename = "clients")]
  clients: Option<Vec<::models::NfsNlmSessionsSession>>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

