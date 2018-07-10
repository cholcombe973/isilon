#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryFileStatistic {
    /// Nodes allocated for the sync action.
    #[serde(rename = "allocated")]
    pub allocated: i32,
    /// An ID for a single performance report.
    #[serde(rename = "id")]
    pub id: i32,
    /// Sync action limit.
    #[serde(rename = "limit")]
    pub limit: i32,
    /// Timestamp for the performance report.
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    /// Total usage for the performance report.
    #[serde(rename = "total")]
    pub total: i32,
}
