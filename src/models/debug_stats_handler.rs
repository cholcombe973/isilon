#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugStatsHandler {
    /// Per-method statistics.
    #[serde(rename = "DELETE")]
    pub delete: Option<::models::DebugStatsUnknown>,
    /// Per-method statistics.
    #[serde(rename = "GET")]
    pub get: Option<::models::DebugStatsUnknown>,
    /// Per-method statistics.
    #[serde(rename = "HEAD")]
    pub head: Option<::models::DebugStatsUnknown>,
    /// Per-method statistics.
    #[serde(rename = "POST")]
    pub post: Option<::models::DebugStatsUnknown>,
    /// Per-method statistics.
    #[serde(rename = "PUT")]
    pub put: Option<::models::DebugStatsUnknown>,
    /// Per-method statistics.
    #[serde(rename = "UNSUPPORTED")]
    pub unsupported: Option<::models::DebugStatsUnknown>,
    /// The URI.
    #[serde(rename = "name")]
    pub name: Option<String>,
}
