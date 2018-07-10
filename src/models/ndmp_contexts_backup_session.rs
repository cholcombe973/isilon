#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpContextsBackupSession {
    /// bre context id; This is not applicable to restore sessions.
    #[serde(rename = "bre_context_id")]
    pub bre_context_id: Option<String>,
    /// Session ID
    #[serde(rename = "session_id")]
    pub session_id: Option<String>,
    /// Session creation time
    #[serde(rename = "start_time")]
    pub start_time: Option<i32>,
    /// The status of the session
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Stream ID
    #[serde(rename = "stream_id")]
    pub stream_id: Option<String>,
}
