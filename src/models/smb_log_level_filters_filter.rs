#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbLogLevelFiltersFilter {
    /// Unique ID of the log filter.
    #[serde(rename = "id")]
    pub id: Option<i32>,
    /// Array of client IP addresses to filter against.
    #[serde(rename = "ip_addrs")]
    pub ip_addrs: Option<Vec<String>>,
    /// Logging level of the filter.
    #[serde(rename = "level")]
    pub level: String,
    /// Array of SMB operations to filter against.
    #[serde(rename = "ops")]
    pub ops: Option<Vec<String>>,
}
