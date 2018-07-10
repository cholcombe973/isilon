

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditProgress {
  /// Lists the current audit log times.
  #[serde(rename = "progress")]
  progress: Option<::models::AuditProgressProgress>
}

