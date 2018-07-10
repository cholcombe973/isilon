

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbSessions {
  /// Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options).
  #[serde(rename = "resume")]
  resume: Option<String>,
  #[serde(rename = "sessions")]
  sessions: Option<Vec<::models::SmbSession>>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

