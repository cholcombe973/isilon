

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditSettings {
  /// Per-Access Zone Audit settings
  #[serde(rename = "settings")]
  settings: Option<::models::AuditSettingsSettings>
}

