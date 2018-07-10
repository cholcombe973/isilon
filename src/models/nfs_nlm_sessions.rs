

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsNlmSessions {
  #[serde(rename = "sessions")]
  sessions: Option<Vec<::models::NfsNlmSessionsSession>>
}

