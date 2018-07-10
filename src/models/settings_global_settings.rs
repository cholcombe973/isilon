

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsGlobalSettings {
  /// Specifies zones that are audited when the protocol_auditing_enabled property is enabled.
  #[serde(rename = "audited_zones")]
  audited_zones: Option<Vec<String>>,
  /// Specifies that events past a certain date are forwarded by the audit CEE forwarder. Format these events as follows: 'Topic@YYYY-MM-DD HH:MM:SS'.
  #[serde(rename = "cee_log_time")]
  cee_log_time: Option<String>,
  /// Specifies a list of Common Event Enabler (CEE) server URIs. Protocol audit logs are sent to these URIs for external processing.
  #[serde(rename = "cee_server_uris")]
  cee_server_uris: Option<Vec<String>>,
  /// Specifies whether logging for API configuration changes are enabled.
  #[serde(rename = "config_auditing_enabled")]
  config_auditing_enabled: Option<bool>,
  /// Specifies whether configuration audit syslog messages are forwarded.
  #[serde(rename = "config_syslog_enabled")]
  config_syslog_enabled: Option<bool>,
  /// Specifies the hostname that is reported in protocol events from this cluster.
  #[serde(rename = "hostname")]
  hostname: Option<String>,
  /// Specifies if logging for the I/O stream is enabled.
  #[serde(rename = "protocol_auditing_enabled")]
  protocol_auditing_enabled: Option<bool>,
  /// Specifies that events past a specified date are forwarded by the audit syslog forwarder. Format these events as follows: 'Topic@YYYY-MM-DD HH:MM:SS' format
  #[serde(rename = "syslog_log_time")]
  syslog_log_time: Option<String>
}

