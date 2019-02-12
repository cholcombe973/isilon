#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsCurrent {
    #[serde(rename = "stats")]
    pub stats: Option<Vec <crate::models::StatisticsCurrentStat>>,
}
