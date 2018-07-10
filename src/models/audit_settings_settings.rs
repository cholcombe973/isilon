#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditSettingsSettings {
    /// Filter of protocol operations to Audit when they fail.
    #[serde(rename = "audit_failure")]
    pub audit_failure: Option<Vec<String>>,
    /// Filter of protocol operations to Audit when they succeed.
    #[serde(rename = "audit_success")]
    pub audit_success: Option<Vec<String>>,
    /// Filter of Audit operations to forward to syslog.
    #[serde(rename = "syslog_audit_events")]
    pub syslog_audit_events: Option<Vec<String>>,
    /// Enables forwarding of events to syslog.
    #[serde(rename = "syslog_forwarding_enabled")]
    pub syslog_forwarding_enabled: Option<bool>,
}
