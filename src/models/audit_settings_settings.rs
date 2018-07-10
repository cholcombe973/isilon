

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditSettingsSettings {
  /// Filter of protocol operations to Audit when they fail.
  #[serde(rename = "audit_failure")]
  audit_failure: Option<Vec<String>>,
  /// Filter of protocol operations to Audit when they succeed.
  #[serde(rename = "audit_success")]
  audit_success: Option<Vec<String>>,
  /// Filter of Audit operations to forward to syslog.
  #[serde(rename = "syslog_audit_events")]
  syslog_audit_events: Option<Vec<String>>,
  /// Enables forwarding of events to syslog.
  #[serde(rename = "syslog_forwarding_enabled")]
  syslog_forwarding_enabled: Option<bool>
}

