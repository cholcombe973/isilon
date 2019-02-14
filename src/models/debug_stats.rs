/// DebugStats : Statistics for all the methods of all URIs in the Platform API.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugStats {
    /// Per-method statistics.
    #[serde(rename = "DESCRIBE")]
    pub describe: Option <crate::models::DebugStatsUnknown>,
    /// Per-method statistics.
    #[serde(rename = "UNKNOWN")]
    pub unknown: Option <crate::models::DebugStatsUnknown>,
    #[serde(rename = "handlers")]
    pub handlers: Option<Vec <crate::models::DebugStatsHandler>>,
}
