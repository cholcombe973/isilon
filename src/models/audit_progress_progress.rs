#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditProgressProgress {
    /// Specifies the last protocol audit event time consumed by the CEE forwarder.
    #[serde(rename = "protocol_audit_cee_time")]
    pub protocol_audit_cee_time: Option<i32>,
    /// Specifies the last logged audit protocol event time.
    #[serde(rename = "protocol_audit_log_time")]
    pub protocol_audit_log_time: Option<i32>,
    /// Specifies the last protocol audit event time consumed by the Syslog forwarder.
    #[serde(rename = "protocol_audit_syslog_time")]
    pub protocol_audit_syslog_time: Option<i32>,
}
