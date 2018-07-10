
/// NdmpContextsBackup : View a NDMP Context

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpContextsBackup {
  /// Context expiration time
  #[serde(rename = "context_expiration_time")]
  context_expiration_time: Option<i32>,
  /// Context ID
  #[serde(rename = "context_id")]
  context_id: Option<String>,
  /// Unique display id.
  #[serde(rename = "id")]
  id: Option<String>,
  /// The directory of the backup. This is not applicable to restore contexts.
  #[serde(rename = "path")]
  path: Option<String>,
  #[serde(rename = "sessions")]
  sessions: Option<Vec<::models::NdmpContextsBackupSession>>,
  /// Snapshot ID reserved for the context. This is not applicable to restore contexts.
  #[serde(rename = "snapid")]
  snapid: Option<i32>,
  /// Context creation time
  #[serde(rename = "start_time")]
  start_time: Option<i32>,
  /// Whether the context is active.
  #[serde(rename = "status")]
  status: Option<String>,
  /// The number of sessions in the context
  #[serde(rename = "total_sessions")]
  total_sessions: Option<i32>,
  /// Type of context; It can be bre, backup, and restore
  #[serde(rename = "type")]
  _type: Option<String>
}

