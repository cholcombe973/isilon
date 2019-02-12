#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryFile {
    #[serde(rename = "statistics")]
    pub statistics: Option<Vec <crate::models::HistoryFileStatistic>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
