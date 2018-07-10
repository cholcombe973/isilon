#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbSharesSummarySummary {
    /// Total number of shares.
    #[serde(rename = "count")]
    pub count: i32,
}
