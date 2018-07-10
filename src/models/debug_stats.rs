
/// DebugStats : Statistics for all the methods of all URIs in the Platform API.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugStats {
  /// Per-method statistics.
  #[serde(rename = "DESCRIBE")]
  describe: Option<::models::DebugStatsUnknown>,
  /// Per-method statistics.
  #[serde(rename = "UNKNOWN")]
  unknown: Option<::models::DebugStatsUnknown>,
  #[serde(rename = "handlers")]
  handlers: Option<Vec<::models::DebugStatsHandler>>
}

