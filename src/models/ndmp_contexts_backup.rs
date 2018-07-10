/// NdmpContextsBackup : View a NDMP Context

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpContextsBackup {
    /// Context expiration time
    #[serde(rename = "context_expiration_time")]
    pub context_expiration_time: Option<i32>,
    /// Context ID
    #[serde(rename = "context_id")]
    pub context_id: Option<String>,
    /// Unique display id.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The directory of the backup. This is not applicable to restore contexts.
    #[serde(rename = "path")]
    pub path: Option<String>,
    #[serde(rename = "sessions")]
    pub sessions: Option<Vec<::models::NdmpContextsBackupSession>>,
    /// Snapshot ID reserved for the context. This is not applicable to restore contexts.
    #[serde(rename = "snapid")]
    pub snapid: Option<i32>,
    /// Context creation time
    #[serde(rename = "start_time")]
    pub start_time: Option<i32>,
    /// Whether the context is active.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// The number of sessions in the context
    #[serde(rename = "total_sessions")]
    pub total_sessions: Option<i32>,
    /// Type of context; It can be bre, backup, and restore
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
