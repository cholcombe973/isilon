#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsHistory {
    #[serde(rename = "stats")]
    pub stats: Option<Vec <crate::models::StatisticsHistoryStat>>,
}
