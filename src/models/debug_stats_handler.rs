#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugStatsHandler {
    /// Per-method statistics.
    #[serde(rename = "DELETE")]
    pub delete: Option <crate::models::DebugStatsUnknown>,
    /// Per-method statistics.
    #[serde(rename = "GET")]
    pub get: Option <crate::models::DebugStatsUnknown>,
    /// Per-method statistics.
    #[serde(rename = "HEAD")]
    pub head: Option <crate::models::DebugStatsUnknown>,
    /// Per-method statistics.
    #[serde(rename = "POST")]
    pub post: Option <crate::models::DebugStatsUnknown>,
    /// Per-method statistics.
    #[serde(rename = "PUT")]
    pub put: Option <crate::models::DebugStatsUnknown>,
    /// Per-method statistics.
    #[serde(rename = "UNSUPPORTED")]
    pub unsupported: Option <crate::models::DebugStatsUnknown>,
    /// The URI.
    #[serde(rename = "name")]
    pub name: Option<String>,
}
