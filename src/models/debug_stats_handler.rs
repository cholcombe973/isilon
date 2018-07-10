

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugStatsHandler {
  /// Per-method statistics.
  #[serde(rename = "DELETE")]
  delete: Option<::models::DebugStatsUnknown>,
  /// Per-method statistics.
  #[serde(rename = "GET")]
  get: Option<::models::DebugStatsUnknown>,
  /// Per-method statistics.
  #[serde(rename = "HEAD")]
  head: Option<::models::DebugStatsUnknown>,
  /// Per-method statistics.
  #[serde(rename = "POST")]
  post: Option<::models::DebugStatsUnknown>,
  /// Per-method statistics.
  #[serde(rename = "PUT")]
  put: Option<::models::DebugStatsUnknown>,
  /// Per-method statistics.
  #[serde(rename = "UNSUPPORTED")]
  unsupported: Option<::models::DebugStatsUnknown>,
  /// The URI.
  #[serde(rename = "name")]
  name: Option<String>
}

