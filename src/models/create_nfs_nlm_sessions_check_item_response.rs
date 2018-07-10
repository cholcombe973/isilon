

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNfsNlmSessionsCheckItemResponse {
  /// Number of lock-loss events detected
  #[serde(rename = "count")]
  count: i32
}

