

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpContextsBackupSession {
  /// bre context id; This is not applicable to restore sessions.
  #[serde(rename = "bre_context_id")]
  bre_context_id: Option<String>,
  /// Session ID
  #[serde(rename = "session_id")]
  session_id: Option<String>,
  /// Session creation time
  #[serde(rename = "start_time")]
  start_time: Option<i32>,
  /// The status of the session
  #[serde(rename = "status")]
  status: Option<String>,
  /// Stream ID
  #[serde(rename = "stream_id")]
  stream_id: Option<String>
}

