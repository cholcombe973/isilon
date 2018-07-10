#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressGlobalProgress {
    /// Specifies the time of the last logged audit protocol event time on the cluster.
    #[serde(rename = "protocol_audit_latest_log_time")]
    pub protocol_audit_latest_log_time: Option<i32>,
    /// Specifies the time of the oldest protocol audit event consumed by the CEE forwarder on the cluster.
    #[serde(rename = "protocol_audit_oldest_cee_time")]
    pub protocol_audit_oldest_cee_time: Option<i32>,
    /// Specifies the time of the oldest protocol audit event consumed by the Syslog forwarder on the cluster.
    #[serde(rename = "protocol_audit_oldest_syslog_time")]
    pub protocol_audit_oldest_syslog_time: Option<i32>,
}
