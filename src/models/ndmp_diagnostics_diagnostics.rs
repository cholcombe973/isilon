

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpDiagnosticsDiagnostics {
  /// Diagnostics level for ndmp.
  #[serde(rename = "diag_level")]
  diag_level: Option<i32>,
  /// The version of the ndmp protocol.
  #[serde(rename = "protocol_version")]
  protocol_version: Option<i32>,
  /// Trace level for ndmp.
  #[serde(rename = "trace_level")]
  trace_level: Option<String>
}

